use std::{
    error::Error,
    fmt::Display,
    io::{BufWriter, Cursor},
};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use common::{Color, Position, Rectangle, Size};
#[allow(unused_imports)]
use image::{EncodableLayout, ImageBuffer, ImageError, ImageFormat, Rgba, RgbaImage};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "wasm")]
use web_sys::ImageData;

/// This struct abstracts the implementation of the actual image away.
/// It's currently using image-rs under the hood but that is an implementation detail.
///
/// Every creation or cloning of the image struct should be available in debug logs
/// because memory allocation is a potential bottle neck.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(PartialEq, Debug)]
pub struct Image {
    pub(crate) buf: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Image {
    /// Creates a new empty image with the given size.
    ///
    /// # Example
    ///
    /// ```
    /// use imagine::Image;
    ///
    /// let my_image = Image::new(100, 200);
    /// let (w, h): (u32, u32) = my_image.size().into();
    /// assert_eq!(w, 100);
    /// assert_eq!(h, 200);
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        log::debug!("Creating image {} x {}", width, height);
        let buf = ImageBuffer::new(width, height);
        Image { buf }
    }

    pub fn new_from_color(width: u32, height: u32, color: &Color) -> Self {
        log::debug!("Creating image {} x {}", width, height);
        let buf = ImageBuffer::from_pixel(width, height, (*color).into());
        Image { buf }
    }

    pub fn size(&self) -> Size {
        Size {
            width: self.buf.width(),
            height: self.buf.height(),
        }
    }

    pub fn new_stamp(color: &Color, hardness: f64, radius: f64) -> Self {
        let mut stamp = Image::new((2. * radius) as u32, (2. * radius) as u32);
        let middle = Position::new(radius as i32, radius as i32);
        let inner_radius = radius * hardness;
        stamp.clean(&Rectangle::new(0, 0, stamp.width(), stamp.height()));
        for x in 0..stamp.width() {
            for y in 0..stamp.height() {
                let dis = middle.distance_to(&(x as i32, y as i32).into());

                if dis <= inner_radius {
                    stamp.buf.put_pixel(x, y, color.with_alpha(255u8).into());
                } else if dis <= radius {
                    let alpha = (1. - ((dis - inner_radius) / (radius - inner_radius))).powf(3.);
                    let alpha = (alpha * 255.) as u8;
                    stamp.buf.put_pixel(x, y, color.with_alpha(alpha).into());
                }
            }
        }
        stamp
    }

    /// Creates a 2x2 image using the given color strings (for test purposes)
    ///
    /// ```no_code
    /// +---+---+
    /// | a | b |
    /// +---+---+
    /// | c | d |
    /// +---+---+
    /// ```
    pub fn new_four_pixels(a: &str, b: &str, c: &str, d: &str) -> Self {
        let mut buf = ImageBuffer::new(2, 2);
        let a: Color = serde_json::from_str(&format!("\"{}\"", a)).unwrap();
        let b: Color = serde_json::from_str(&format!("\"{}\"", b)).unwrap();
        let c: Color = serde_json::from_str(&format!("\"{}\"", c)).unwrap();
        let d: Color = serde_json::from_str(&format!("\"{}\"", d)).unwrap();
        buf.put_pixel(0, 0, a.into());
        buf.put_pixel(1, 0, b.into());
        buf.put_pixel(1, 1, c.into());
        buf.put_pixel(0, 1, d.into());
        Image { buf }
    }

    /// Creates a new image with the content of the given image file.
    #[allow(dead_code)]
    pub fn from_file(path: &str) -> Result<Self, ImageError> {
        let img = image::open(path)?;
        let buf = img.to_rgba8();
        Ok(Image { buf })
    }

    pub fn from_bytes(buffer: &[u8]) -> Result<Self, ImageError> {
        let img = image::load_from_memory(buffer)?;
        let buf = img.to_rgba8();
        Ok(Image { buf })
    }

    pub fn into_array(&self) -> &Vec<u8> {
        self.buf.as_raw()
    }

    pub fn into_png_bytes(&self) -> Vec<u8> {
        let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
        self.buf.write_to(&mut buffer, ImageFormat::Png).unwrap();

        let bytes: Vec<u8> = buffer.into_inner().unwrap().into_inner();
        bytes
    }

    pub fn from_base64(base64: &str) -> Result<Self, Box<dyn Error>> {
        let buf = STANDARD.decode(base64)?;
        let img = image::load_from_memory(&buf)?;
        let buf = img.to_rgba8();
        Ok(Image { buf })
    }

    #[allow(dead_code)]
    pub fn save(&self, path: &str) -> Result<(), ImageError> {
        self.buf.save(path)
    }

    pub fn encode_base64(&self) -> Result<String, ImageError> {
        let mut buf: Vec<u8> = Vec::new();
        self.buf
            .write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)?;
        let encoded = STANDARD.encode(&buf);
        Ok(encoded)
    }

    pub fn height(&self) -> u32 {
        self.buf.height()
    }

    pub fn width(&self) -> u32 {
        self.buf.width()
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        self.buf.to_vec()
    }

    pub fn pixel(&self, x: u32, y: u32) -> Color {
        self.buf.get_pixel(x, y).into()
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, g: Color) {
        self.buf.put_pixel(x, y, g.into());
    }

    pub fn clean(&mut self, area: &Rectangle) {
        for i in area.points() {
            if i.x.is_positive()
                && i.x < self.width() as i32
                && i.y.is_positive()
                && i.y < self.height() as i32
            {
                self.buf
                    .put_pixel(i.x as u32, i.y as u32, Color::TRANSPARENT.into());
            }
        }
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        if log::log_enabled!(log::Level::Debug) {
            let (width, height): (u32, u32) = self.size().into();
            log::debug!("Cloning image {} x {}", width, height);
        }
        let buf = self.buf.clone();
        Image { buf }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[image {}x{}]", self.buf.width(), self.buf.height())
    }
}

#[cfg(feature = "wasm")]
impl From<&Image> for ImageData {
    fn from(value: &Image) -> ImageData {
        ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(value.buf.as_bytes()),
            value.width(),
            value.height(),
        )
        .unwrap()
    }
}

#[cfg(feature = "wasm")]
impl From<ImageData> for Image {
    fn from(value: ImageData) -> Self {
        let converted: Option<ImageBuffer<Rgba<u8>, Vec<u8>>> =
            RgbaImage::from_raw(value.width(), value.height(), value.data().to_vec());
        let buf = converted.expect("Oh no");
        Image { buf }
    }
}
