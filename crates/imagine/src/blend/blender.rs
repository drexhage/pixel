use common::{Position, Rectangle};

use crate::{BlendMode, Image};

/// A blender blends multiple images together (composition)
pub trait Blender {
    /// Name of the blender
    fn name(&self) -> &'static str;

    /// Marks that the image with the given marker won't change until the next call of the clean method.
    /// That is important for caching these images into texture if hardware acceleration is used.
    fn load(&mut self, marker: usize, img: &Image);

    /// Cleans the cache and thus forgets all images that got loaded through the load method.
    fn clean(&mut self);

    /// Blends `overlay` on top of `base` with the given mode, positions and alphas.
    /// Creates a new image that is defined by `destination`.
    fn blend(
        &mut self,
        mode: BlendMode,
        destination: &Rectangle,
        overlay: (&Image, Position, f64, Option<usize>),
        base: (&Image, Position, f64, Option<usize>),
    ) -> Image;

    /// Blends `overlay` on top of `base` with the given mode, positions and alphas.
    /// Renders only the damaged area defined by `destination` and adjusts base accordingly.
    fn blend_damaged_into(
        &mut self,
        mode: BlendMode,
        damage: &Rectangle,
        destination: (&mut Image, Position),
        overlay: (&Image, Position, f64),
        base: (&Image, Position, f64),
    );

    /// Blends the active image on top of the base image by mutating the passed reference
    fn blend_damaged(
        &mut self,
        mode: BlendMode,
        base: (&mut Image, Position, f64),
        overlay: (&Image, Position, f64),
        damage: &Rectangle,
    );

    /// Blends all given children based on their properties into one image that fits the destination
    fn blend_all(&mut self, destination: &Rectangle, children: Vec<BlendLayer>) -> Image {
        let (width, height) = (destination.size.width, destination.size.height);
        let mut result = Image::new(width, height);
        for (mode, img, pos, alpha, visible, marker) in children {
            if !visible {
                continue;
            }
            result = self.blend(
                mode,
                destination,
                (img, pos, alpha, marker),
                (&result, destination.position, 1.0, None),
            );
        }
        result
    }
}

pub type BlendLayer<'a> = (BlendMode, &'a Image, Position, f64, bool, Option<usize>);
