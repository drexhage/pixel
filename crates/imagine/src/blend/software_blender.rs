use common::{Position, Rectangle};
use image::{ImageBuffer, Rgba};

use crate::{BlendMode, Blender, Image};

pub struct SoftwareBlender {}

/// SoftwareBlender is a basic reference [Blender] that isn't hardware-accelerated.
impl Blender for SoftwareBlender {
    fn name(&self) -> &'static str {
        "Software"
    }

    fn blend(
        &mut self,
        mode: BlendMode,
        destination: &Rectangle,
        (overlay_img, overlay_pos, overlay_alpha, _overlay_id): (
            &Image,
            Position,
            f64,
            Option<usize>,
        ),
        (base_img, base_pos, base_alpha, _base_id): (&Image, Position, f64, Option<usize>),
    ) -> Image {
        let mut buf = ImageBuffer::new(destination.size.width, destination.size.height);
        let blending_function = |b: &Rgba<u8>, a: &Rgba<u8>| -> Rgba<u8> {
            Self::blend_pixel(mode, b, a, base_alpha, overlay_alpha)
        };
        let (x_base, y_base) = base_pos.into();
        let (x_overlay, y_overlay) = overlay_pos.into();
        for position in destination.points() {
            let (x, y) = position.into();
            // transform into base/active coordinates
            let b_x = x - x_base;
            let b_y = y - y_base;
            let a_x = x - x_overlay;
            let a_y = y - y_overlay;
            let i_x = x - destination.position.x;
            let i_y = y - destination.position.y;
            // blend
            let base_pixel = if b_x >= 0
                && b_x < base_img.width() as i32
                && b_y >= 0
                && b_y < base_img.height() as i32
            {
                base_img.buf.get_pixel(b_x as u32, b_y as u32)
            } else {
                &Rgba([0, 0, 0, 0])
            };
            let active_pixel = if a_x >= 0
                && a_x < overlay_img.width() as i32
                && a_y >= 0
                && a_y < overlay_img.height() as i32
            {
                overlay_img.buf.get_pixel(a_x as u32, a_y as u32)
            } else {
                &Rgba([0, 0, 0, 0])
            };
            let blended_pixel = blending_function(base_pixel, active_pixel);
            buf.put_pixel(i_x as u32, i_y as u32, blended_pixel);
        }
        Image { buf }
    }

    fn blend_damaged_into(
        &mut self,
        mode: BlendMode,
        damage: &Rectangle,
        (destination_img, destination_pos): (&mut Image, Position),
        (overlay_img, overlay_pos, overlay_alpha): (&Image, Position, f64),
        (base_img, base_pos, base_alpha): (&Image, Position, f64),
    ) {
        let blending_function = |b: &Rgba<u8>, a: &Rgba<u8>| -> Rgba<u8> {
            Self::blend_pixel(mode, b, a, base_alpha, overlay_alpha)
        };

        let active_area = Rectangle::of(overlay_pos, overlay_img.size());
        let base_area = Rectangle::of(base_pos, base_img.size());
        let dest_area = Rectangle::of(destination_pos, destination_img.size());

        let damage = &Rectangle::intersectn(&[damage, &base_area, &active_area, &dest_area]);

        for position in damage.points() {
            let b = position - base_pos;
            let a = position - overlay_pos;
            let i = position - destination_pos;
            let blended_pixel = blending_function(
                base_img.buf.get_pixel(b.x as u32, b.y as u32),
                overlay_img.buf.get_pixel(a.x as u32, a.y as u32),
            );
            destination_img
                .buf
                .put_pixel(i.x as u32, i.y as u32, blended_pixel);
        }
    }

    fn blend_damaged(
        &mut self,
        mode: BlendMode,
        (base_img, base_pos, base_alpha): (&mut Image, Position, f64),
        (overlay_img, overlay_pos, overlay_alpha): (&Image, Position, f64),
        damage: &Rectangle,
    ) {
        let blending_function = |b: &Rgba<u8>, a: &Rgba<u8>| -> Rgba<u8> {
            Self::blend_pixel(mode, b, a, base_alpha, overlay_alpha)
        };
        let (x_base, y_base) = base_pos.into();
        let (x_overlay, y_overlay) = overlay_pos.into();
        for position in damage.points() {
            let (x, y) = position.into();
            // transform into base/active coordinates
            let b_x = x - x_base;
            let b_y = y - y_base;
            let a_x = x - x_overlay;
            let a_y = y - y_overlay;

            let base_pixel = if b_x >= 0
                && b_x < base_img.width() as i32
                && b_y >= 0
                && b_y < base_img.height() as i32
            {
                base_img.buf.get_pixel(b_x as u32, b_y as u32)
            } else {
                &Rgba([0, 0, 0, 0])
            };
            let active_pixel = if a_x >= 0
                && a_x < overlay_img.width() as i32
                && a_y >= 0
                && a_y < overlay_img.height() as i32
            {
                overlay_img.buf.get_pixel(a_x as u32, a_y as u32)
            } else {
                &Rgba([0, 0, 0, 0])
            };
            // let active_pixel = active.0.get_pixel(a_x as u32, a_y as u32);
            let blended_pixel = blending_function(base_pixel, active_pixel);
            base_img.put_pixel(b_x as u32, b_y as u32, (&blended_pixel).into());
        }
    }

    fn load(&mut self, _marker: usize, _img: &Image) {
        // not needed
    }

    fn clean(&mut self) {
        // not needed
    }
}

impl Default for SoftwareBlender {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftwareBlender {
    pub fn new() -> Self {
        SoftwareBlender {}
    }

    pub fn blend_pixel(
        blend_mode: BlendMode,
        b: &Rgba<u8>,
        a: &Rgba<u8>,
        alpha_base: f64,
        alpha_active: f64,
    ) -> Rgba<u8> {
        match blend_mode {
            BlendMode::Alpha => {
                let alpha_a = (a[3] as f64 / 255.0) * alpha_active;
                let alpha_b = (b[3] as f64 / 255.0) * alpha_base;
                let alpha = alpha_a + alpha_b * (1.0 - alpha_a);
                let mut result = Rgba([0, 0, 0, (alpha * 255.0) as u8]);
                for i in 0..3 {
                    let c_a = a[i] as f64 / 255.0;
                    let c_b = b[i] as f64 / 255.0;
                    result[i] =
                        (((c_a * alpha_a + c_b * alpha_b * (1.0 - alpha_a)) / alpha) * 255.0) as u8;
                }
                result
            }
            BlendMode::Remove => {
                let mut result = *b;
                let alpha_a = (a[3] as f64 / 255.0) * alpha_active;
                let alpha_b = (b[3] as f64 / 255.0) * alpha_base;
                result.0[3] = if alpha_b < alpha_a {
                    0
                } else {
                    ((alpha_b - alpha_a) * 255.0) as u8
                };
                result
            }
            BlendMode::Darken => {
                let alpha_a = (a[3] as f64 / 255.0) * alpha_active;
                let alpha_b = (b[3] as f64 / 255.0) * alpha_base;
                let mut result = *b;
                for i in 0..3 {
                    let c_a = a[i] as f64 / 255.0;
                    let c_b = b[i] as f64 / 255.0;
                    let factor = f64::min(c_a * alpha_b * alpha_a, c_b * alpha_a * alpha_b)
                        + alpha_a * c_a * (1. - alpha_b)
                        + alpha_b * c_b * (1. - alpha_a);
                    result[i] = (factor * 255.) as u8;
                }
                result[3] = ((alpha_a + alpha_b - alpha_a * alpha_b) * 255.) as u8;
                result
            }
            BlendMode::Lighten => {
                let alpha_a = (a[3] as f64 / 255.0) * alpha_active;
                let alpha_b = (b[3] as f64 / 255.0) * alpha_base;
                let mut result = *b;
                for i in 0..3 {
                    let c_a = a[i] as f64 / 255.0;
                    let c_b = b[i] as f64 / 255.0;
                    let factor = f64::max(c_a * alpha_b * alpha_a, c_b * alpha_a * alpha_b)
                        + alpha_a * c_a * (1. - alpha_b)
                        + alpha_b * c_b * (1. - alpha_a);
                    result[i] = (factor * 255.) as u8;
                }
                result[3] = ((alpha_a + alpha_b - alpha_a * alpha_b) * 255.) as u8;
                result
            }
            BlendMode::Screen => {
                let mut result = *b;
                let sa = alpha_active;
                let da = alpha_base;
                for i in 0..3 {
                    let sc = a[i] as f64 / 255.0;
                    let dc = b[i] as f64 / 255.0;
                    let sca = sc * sa;
                    let dca = dc * da;
                    result[i] = ((sca + dca - sca * dca) * 255.) as u8;
                }
                result[3] = ((sa + da - sa * da) * 255.) as u8;
                result
            }
        }
    }
}

#[cfg(test)]
mod test {
    use image::Rgba;

    use crate::{Blender, Image, SoftwareBlender};

    use super::BlendMode;

    #[test]
    fn alpha_blending_empty_pixels_stay_empty() {
        let a = Rgba([0, 0, 0, 0]);
        let b = Rgba([0, 0, 0, 0]);
        let res1 = SoftwareBlender::blend_pixel(BlendMode::Alpha, &b, &a, 1.0, 1.0);
        let res2 = SoftwareBlender::blend_pixel(BlendMode::Alpha, &a, &b, 1.0, 1.0);
        assert_eq!(res1, res2);
        assert_eq!(res1, Rgba([0, 0, 0, 0]));
    }

    #[test]
    fn alpha_blending_completely_opaque_active() {
        let a = Rgba([10, 20, 42, 255]);
        let b = Rgba([0, 0, 0, 0]);
        let res1 = SoftwareBlender::blend_pixel(BlendMode::Alpha, &b, &a, 1.0, 1.0);
        assert_eq!(res1, Rgba([10, 20, 42, 255]));
    }

    #[test]
    fn alpha_blending_completely_transparent_active() {
        let a = Rgba([0, 0, 0, 0]);
        let b = Rgba([10, 20, 42, 255]);
        let res1 = SoftwareBlender::blend_pixel(BlendMode::Alpha, &b, &a, 1.0, 1.0);
        assert_eq!(res1, Rgba([10, 20, 42, 255]));
    }

    #[test]
    fn alpha_blending() {
        let base = Image::new_four_pixels("#ffffffff", "#ffffffff", "#ffffff00", "#ffffffff");
        let active = Image::new_four_pixels("#00ffff00", "#00ffff00", "#00ffff00", "#00ffffff");
        let mut dest = Image::new(2, 2);
        let mut blender = SoftwareBlender::new();
        blender.blend_damaged_into(
            BlendMode::Alpha,
            &(0, 0, 2, 2).into(),
            (&mut dest, (0, 0).into()),
            (&active, (0, 0).into(), 1.0),
            (&base, (0, 0).into(), 1.0),
        );
        let expected = Image::new_four_pixels("#ffffffff", "#ffffffff", "#00000000", "#00ffffff");
        assert_eq!(dest, expected);
    }
}
