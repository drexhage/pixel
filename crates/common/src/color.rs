use std::fmt::Display;

#[cfg(feature = "image")]
use image::Rgba;
#[cfg(feature = "serde")]
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(PartialEq, Debug, Clone, Copy)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{:02x}{:02x}{:02x}{:02x}",
            self.r, self.g, self.b, self.a
        )
    }
}

#[cfg(feature = "image")]
impl From<Color> for Rgba<u8> {
    fn from(value: Color) -> Self {
        Rgba([value.r, value.g, value.b, value.a])
    }
}

#[cfg(feature = "image")]
impl From<&Rgba<u8>> for Color {
    fn from(value: &Rgba<u8>) -> Self {
        Color {
            r: value.0[0],
            g: value.0[1],
            b: value.0[2],
            a: value.0[3],
        }
    }
}

impl Color {
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };

    pub fn with_alpha(&self, alpha: u8) -> Self {
        let mut result = *self;
        result.a = alpha;
        result
    }

    pub fn with_added_alpha(&self, alpha: u8) -> Self {
        let mut result = *self;
        result.a = result.a.saturating_add(alpha);
        result
    }
}

// Custom serde for natural serialization (`"#ff0000ff"` instead of `{"r":256,"g":0,"b":0,"a":256}`)

#[cfg(feature = "serde")]
impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = format!("{}", &self);
        serializer.serialize_str(&format)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PositionVisitor;

        impl<'de> Visitor<'de> for PositionVisitor {
            type Value = Color;

            fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                todo!()
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if !v.starts_with('#') {
                    return Err(de::Error::custom("Color has to start with a '#'."));
                }
                let err_map = |_| de::Error::custom("Invalid hex.");
                let radix = 16;
                match v.len() {
                    // "#f00"
                    4 => {
                        let r = u8::from_str_radix(&v[1..2], radix).map_err(err_map)?;
                        let g = u8::from_str_radix(&v[2..3], radix).map_err(err_map)?;
                        let b = u8::from_str_radix(&v[3..4], radix).map_err(err_map)?;
                        Ok(Color {
                            r: 16 * r + r,
                            g: 16 * g + g,
                            b: 16 * b + b,
                            a: 255,
                        })
                    }
                    // "#ff0000"
                    7 => {
                        let r = u8::from_str_radix(&v[1..3], radix).map_err(err_map)?;
                        let g = u8::from_str_radix(&v[3..5], radix).map_err(err_map)?;
                        let b = u8::from_str_radix(&v[5..7], radix).map_err(err_map)?;
                        Ok(Color { r, g, b, a: 255 })
                    }
                    // "#ff0000ff"
                    9 => {
                        let r = u8::from_str_radix(&v[1..3], radix).map_err(err_map)?;
                        let g = u8::from_str_radix(&v[3..5], radix).map_err(err_map)?;
                        let b = u8::from_str_radix(&v[5..7], radix).map_err(err_map)?;
                        let a = u8::from_str_radix(&v[7..9], radix).map_err(err_map)?;
                        Ok(Color { r, g, b, a })
                    }
                    _ => Err(de::Error::custom(
                        "Wrong size for color. Length must be one of 4, 7 or 9.",
                    )),
                }
            }
        }

        deserializer.deserialize_str(PositionVisitor)
    }
}

#[cfg(all(feature = "serde", test))]
mod test {
    use crate::Color;

    #[test]
    fn serialize9() {
        let json = serde_json::to_string(&Color::RED).unwrap();
        assert_eq!(json, "\"#ff0000ff\"")
    }

    #[test]
    fn deserialize9() {
        let json = "\"#ff0000ff\"";
        let reconstructed: Color = serde_json::from_str(json).unwrap();
        assert_eq!(reconstructed, Color::RED);
    }

    #[test]
    fn deserialize7() {
        let json = "\"#ff0000\"";
        let reconstructed: Color = serde_json::from_str(json).unwrap();
        assert_eq!(reconstructed, Color::RED);
    }

    #[test]
    fn deserialize4() {
        let json = "\"#f00\"";
        let reconstructed: Color = serde_json::from_str(json).unwrap();
        assert_eq!(reconstructed, Color::RED);
    }

    #[test]
    fn display() {
        let color = Color {
            r: 255,
            g: 128,
            b: 27,
            a: 255,
        };
        let display = format!("{}", color);
        assert_eq!(display, "#ff801bff");
    }
}
