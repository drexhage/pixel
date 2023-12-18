use common::{Position, Rectangle, Size};
use image::imageops;

use crate::{blend::SoftwareBlender, BlendMode, Image};

impl Image {
    pub fn grayscale(&mut self) {
        // TODO
    }

    pub fn gaussian_noise(&mut self, mean: f64, stddev: f64, seed: u64) {
        imageproc::noise::gaussian_noise_mut(&mut self.buf, mean, stddev, seed);
    }

    pub fn flip_horizontally(&mut self) {
        imageops::flip_horizontal_in_place(&mut self.buf);
    }

    pub fn flip_vertically(&mut self) {
        imageops::flip_vertical_in_place(&mut self.buf);
    }

    /// Draws the stamp image at each position of the track on the image struct.
    ///
    /// Returns damaged area in image coordinates.
    pub fn draw_line(&mut self, stamp: &Image, track: &[Position]) -> Rectangle {
        log::debug!("Starting to draw");
        let (width, height) = (self.width() as i32, self.height() as i32);
        let (stamp_w, stamp_h) = (stamp.width() as i32, stamp.height() as i32);

        track
            .iter()
            .map(|a| (a.x, a.y))
            // expand x coordinates
            .map(|(x0, y0)| (x0, y0, (x0 - stamp_w)..(x0 + stamp_w)))
            .flat_map(|(x0, y0, xr)| xr.map(move |q| (x0, y0, q)))
            .filter(|(_x0, _y0, x)| *x >= 0 && *x < width)
            .filter(|(_x0, y0, _x)| y0 - stamp_h < height && y0 + stamp_h > 0)
            // expand y coordinates
            .map(|(x0, y0, x)| (x0, y0, x, (y0 - stamp_h)..(y0 + stamp_h)))
            .flat_map(|(x0, y0, x, yr)| yr.map(move |q| (x0, y0, x, q)))
            .filter(|(_x0, _y0, _x, y)| *y >= 0 && *y < height)
            // map into stamp coordinates
            .map(|(x0, y0, x, y)| (x - (x0 - stamp_w / 2), y - (y0 - stamp_h / 2), x, y))
            .filter(|(x_stamp, y_stamp, _x, _y)| {
                *x_stamp >= 0 && *x_stamp < stamp_w && *y_stamp >= 0 && *y_stamp < stamp_h
            })
            .for_each(|(x_stamp, y_stamp, x, y)| {
                let pixel = stamp.buf.get_pixel(x_stamp as u32, y_stamp as u32);
                let existing = self.buf.get_pixel(x as u32, y as u32);
                let result =
                    SoftwareBlender::blend_pixel(BlendMode::Alpha, existing, pixel, 1.0, 1.0);
                self.buf.put_pixel(x as u32, y as u32, result);
            });

        let half = Position::new(stamp_w / 2, stamp_h / 2);
        return track
            .iter()
            .map(|&pos| (pos - half, Size::new(stamp_w as u32, stamp_h as u32)))
            .map(|(pos, size)| Rectangle::of(pos, size))
            .reduce(|rhs, lhs| Rectangle::bounding(&rhs, &lhs))
            .unwrap_or(Rectangle::new(0, 0, 0, 0));
    }
}
