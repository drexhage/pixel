use common::{Position, Size};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Coordinate(pub f64, pub f64);

/// All coordinates and sizes in this context are always in canvas coordinates and not in image ones!
#[derive(Clone)]
#[wasm_bindgen]
pub struct CanvasDisplay {
    pub position: Position,
    pub scale: f64,
    pub img_size: Size,
    pub canvas_size: Size,
    pub gap: u32,
}

#[wasm_bindgen]
impl CanvasDisplay {
    #[wasm_bindgen(constructor)]
    pub fn constructor(
        img_width: u32,
        img_height: u32,
        canvas_width: u32,
        canvas_height: u32,
        gap: u32,
    ) -> Self {
        Self::initial(
            Size {
                width: img_width,
                height: img_height,
            },
            Size {
                width: canvas_width,
                height: canvas_height,
            },
            gap,
        )
    }

    pub fn initial(img_size: Size, canvas_size: Size, gap: u32) -> Self {
        let mut result = CanvasDisplay {
            scale: 1.0,
            position: Position::zero(),
            img_size,
            canvas_size,
            gap,
        };
        result.center();
        result
    }

    /// Centers the image on the canvas
    ///
    /// There are two major cases regarding the aspect ratios of both canvas and image
    ///
    /// ## `(img_width / img_height) < (canvas_width / canvas_height)`
    ///
    /// ```no_compile
    /// +-----------------+
    /// |      +----+     |
    /// |      |    |     |
    /// |      |    |     |
    /// |      |    |     |
    /// |      +----+     |
    /// +-----------------+
    /// ```
    ///
    /// ## `(img_width / img_height) > (canvas_width / canvas_height)``
    ///
    /// ```no_compile
    /// +-----------------+
    /// |                 |
    /// |+---------------+|
    /// ||               ||
    /// |+---------------+|
    /// |                 |
    /// +-----------------+
    /// ```
    ///
    pub fn center(&mut self) {
        let img_size = &self.img_size;
        let canvas_size = &self.canvas_size;
        let gap = self.gap;
        let img_format = img_size.width as f64 / img_size.height as f64;
        let canvas_format = canvas_size.width as f64 / canvas_size.height as f64;
        let (scale, position) = if img_format > canvas_format {
            // we know that this point: img_width * scale = canvas_width + 2 * gap
            let scale = (canvas_size.width - 2 * gap) as f64 / img_size.width as f64;
            let scaled_image_height = (scale * img_size.height as f64) as u32;
            let y = (canvas_size.height - scaled_image_height) / 2;
            let position = Position::new(gap as i32, y as i32);
            (scale, position)
        } else {
            let scale = (canvas_size.height - 2 * gap) as f64 / img_size.height as f64;
            let scaled_image_width = (scale * img_size.width as f64) as u32;
            let x = (canvas_size.width - scaled_image_width) / 2;
            let position = Position::new(x as i32, gap as i32);
            (scale, position)
        };
        self.scale = scale;
        self.position = position;
    }

    pub fn set_img_size(&mut self, width: u32, height: u32) {
        self.img_size.width = width;
        self.img_size.height = height;
    }

    pub fn set_canvas_size(&mut self, width: u32, height: u32) {
        self.canvas_size.width = width;
        self.canvas_size.height = height;
    }

    pub fn translate_position(&self, x: f64, y: f64) -> Coordinate {
        let x = (x - self.position.x as f64) / self.scale;
        let y = (y - self.position.y as f64) / self.scale;
        Coordinate(x, y)
    }

    /// The point (`x`, `y`) at which is being zoomed should stay at the same spot in canvas coordinates.
    /// Only the surrounding should be zoomed in/out by `zoom_delta`.
    ///
    /// ## Visualization
    ///
    /// The point at which the cursor is is visualized with `o`.
    /// This point stays at the same spot on the canvas while the rest is zoomed in.
    ///
    /// ```no_compile
    /// +---------|-------+    +---------|-------+
    /// |      +--|-+     |    |  |      |  |    |
    /// -------|--o |     |    ---|------o  |    |
    /// |      |    |     | -> |  |         |    |
    /// |      |    |     |    |  |         |    |
    /// |      +----+     |    |  |         |    |
    /// +-----------------+    +-----------------+
    /// ```
    ///
    /// With a given `zoom_delta`, we already know the size of the image.
    /// The only thing we don't know is the new offset of the image in canvas coordinates.
    ///
    /// We know that the fixed point stays the same after the zoom:
    ///
    /// ```no_compile
    /// (fix_x - pos_x) / scale = (fix_x - new_x) / (scale * zoom_delta)
    ///     <=> (fix_x - pos_x) = (fix_x - new_x) / zoom_delta
    ///     <=> (fix_x - pos_x) * zoom_delta = (fix_x - new_x)
    ///     <=> new_x = fix_x - (fix_x - pos_x) * zoom_delta
    /// ```
    ///
    /// Analogous for the y-coordinates.
    pub fn zoom_at_position(&mut self, x: f64, y: f64, zoom_delta: f64) {
        let new_x = x - (x - self.position.x as f64) * zoom_delta;
        let new_y = y - (y - self.position.y as f64) * zoom_delta;
        self.position.x = new_x as i32;
        self.position.y = new_y as i32;
        self.scale *= zoom_delta;
    }

    pub fn move_relative(&mut self, delta_x: i32, delta_y: i32) {
        self.position.x += delta_x;
        self.position.y += delta_y;
    }
}

#[cfg(test)]
mod test {
    use common::Size;

    use crate::CanvasDisplay;

    #[test]
    fn fit_both_sides_no_gap() {
        let img_size = Size::new(123, 432);
        let canvas_size = Size::new(123, 432);
        let canvas_display = CanvasDisplay::initial(img_size, canvas_size, 0);
        assert_eq!(canvas_display.scale, 1.0);
        assert_eq!(canvas_display.position.x, 0);
        assert_eq!(canvas_display.position.y, 0);
    }

    #[test]
    fn fit_one_side_no_gap() {
        let img_size = Size::new(120, 432);
        let canvas_size = Size::new(240, 432);
        let canvas_display = CanvasDisplay::initial(img_size, canvas_size, 0);
        assert_eq!(canvas_display.scale, 1.0);
        assert_eq!(canvas_display.position.x, 60); // (240 - 120) / 2
        assert_eq!(canvas_display.position.y, 0);
    }

    #[test]
    fn too_big_both_sides_no_gap() {
        let img_size = Size::new(120, 400);
        let canvas_size = Size::new(240, 1000);
        let canvas_display = CanvasDisplay::initial(img_size, canvas_size, 0);
        assert_eq!(canvas_display.scale, 2.0);
        assert_eq!(canvas_display.position.x, 0);
        assert_eq!(canvas_display.position.y, 100); // (1000 - (2.0 * 400)) / 2
    }

    #[test]
    fn same_ratio_some_gap() {
        let display = CanvasDisplay::initial(Size::new(100, 100), Size::new(200, 200), 10);
        assert_eq!(display.scale, 1.8); // (400 - 2 * 10) / 200
        assert_eq!(display.position.x, 10);
        assert_eq!(display.position.y, 10);
    }

    #[test]
    fn zoom_at_position_keep_position() {
        let mut display = CanvasDisplay::initial((100, 200).into(), (200, 100).into(), 20);
        let (x, y) = (42., 37.);
        let before = display.translate_position(x, y);
        display.zoom_at_position(x, y, 1.1);
        let after = display.translate_position(x, y);
        assert!(before.0 - after.0 < 0.5);
        assert!(before.1 - after.1 < 0.5);
    }
}
