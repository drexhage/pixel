use serde::{
    de::{self, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};

use super::{image::Image, ImageSource};

// A Image serializes into a struct that represents a ImageDto.
// For that reason the deserialization has to also happen over ImageData instead.

impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let data = self.encode_base64().unwrap();
        let source = ImageSource::Base64Png;
        let mut state: <S as Serializer>::SerializeStruct =
            serializer.serialize_struct("Image", 2)?;
        state.serialize_field("data", &data)?;
        state.serialize_field("src", &source)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ImageVisitor)
    }
}

struct ImageVisitor;

impl<'de> Visitor<'de> for ImageVisitor {
    type Value = Image;

    fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!()
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut src: Option<ImageSource> = None;
        let mut data: Option<String> = None;
        while let Some(key) = access.next_key()? {
            match key {
                "src" => {
                    if src.is_some() {
                        return Err(de::Error::duplicate_field("src"));
                    }
                    src = Some(access.next_value()?);
                }
                "data" => {
                    if data.is_some() {
                        return Err(de::Error::duplicate_field("data"));
                    }
                    data = Some(access.next_value()?);
                }
                _ => {}
            }
        }

        let src = src.ok_or_else(|| de::Error::missing_field("src"))?;
        let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
        if src != ImageSource::Base64Png {
            return Err(de::Error::custom("Can't deserialize a non-base64 image"));
        }

        Image::from_base64(&data).map_err(|_| de::Error::custom("Invalid base64 image"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let img = Image::new(100, 200);
        let json = serde_json::to_string(&img).unwrap();
        let restructed: Image = serde_json::from_str(&json).unwrap();
        assert_eq!(img, restructed);
    }
}
