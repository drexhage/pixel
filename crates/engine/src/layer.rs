use common::{Position, Rectangle};
use imagine::{BlendMode, Image};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

/// Describes all the meta data of a layer
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LayerAttributes {
    pub pos: Position,
    pub mode: BlendMode,
    pub alpha: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(tag = "type")]
pub enum LayerFlag {
    Root,
    Group,
    Pixel,
    Text(String),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GhostImage {
    pub img: Image,
    pub mode: BlendMode,
    pub alpha: f32,
}

/// A single layer
#[derive(PartialEq, Debug, Clone)]
pub struct Layer {
    /// The actual displayed result
    pub img: Image,

    /// Possible intermediate manipulations go here
    /// The ghost rendered on top of img can then be found in the second tuple entry
    pub ghost: Option<GhostImage>,

    pub zombie: Option<Image>,

    pub attr: LayerAttributes,
    pub flag: LayerFlag,
    pub visible: bool,
    pub name: String,
}

impl Serialize for Layer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Layer", 5)?;
        s.serialize_field("attr", &self.attr)?;
        s.serialize_field("visible", &self.visible)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("flag", &self.flag)?;
        s.serialize_field("size", &vec![&self.img.width(), &self.img.height()])?;

        // do not serialize ghost, zombie or img but instead a pointer for preview (for wasm really)
        let ptr = self.img.into_array().as_ptr() as u32;
        s.serialize_field("pointer", &ptr)?;

        s.end()
    }
}

impl Layer {
    /// Creates an empty layer at position (0, 0) using alpha blending with the given size.
    pub fn default(width: u32, height: u32) -> Self {
        let pos: Position = Position { x: 0, y: 0 };
        let img = Image::new(width, height);
        let blend = BlendMode::Alpha;
        let attr = LayerAttributes {
            pos,
            mode: blend,
            alpha: 1.0,
        };
        let flag = LayerFlag::Pixel;
        
        Layer {
            img,
            attr,
            flag,
            ghost: None,
            zombie: None,
            visible: true,
            name: "New Layer".to_string(),
        }
    }

    pub fn rectangle(&self) -> Rectangle {
        (
            self.attr.pos.x,
            self.attr.pos.y,
            self.img.width(),
            self.img.height(),
        )
            .into()
    }

    /// Creates a default layer with the given content at position (0, 0).
    pub fn from_content(content: Image) -> Self {
        let mut result = Self::default(content.width(), content.height());
        result.img = content;
        result
    }

    pub fn is_hit(&self, pos: &Position) -> bool {
        if !self.visible {
            return false;
        }
        let x_image = pos.x - self.attr.pos.x;
        let y_image = pos.y - self.attr.pos.y;
        if x_image < 0 || y_image < 0 {
            return false;
        }
        let x_image = x_image as u32;
        let y_image = y_image as u32;
        if x_image >= self.img.width() || y_image >= self.img.height() {
            return false;
        }
        let pixel = self.img.pixel(x_image, y_image);
        pixel.a != 0
    }
}

#[cfg(test)]
mod test {
    use common::Position;
    use imagine::{BlendMode, Image};

    use super::{Layer, LayerAttributes, LayerFlag};

    #[test]
    fn check_hit() {
        let img = Image::new_four_pixels("#fff", "#ffffff00", "#fff", "#fff");
        let layer = Layer {
            img,
            ghost: None,
            zombie: None,
            attr: LayerAttributes {
                pos: Position::new(1, 1),
                mode: BlendMode::Alpha,
                alpha: 0.9,
            },
            flag: LayerFlag::Pixel,
            visible: true,
            name: "lksdjf".to_string(),
        };
        assert!(!layer.is_hit(&Position::new(0, 0)));
        assert!(layer.is_hit(&Position::new(1, 1)));
        assert!(!layer.is_hit(&Position::new(1, 0)));
    }
}
