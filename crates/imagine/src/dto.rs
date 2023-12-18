use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Image;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub enum ImageSource {
    #[serde(rename = "encode/png")]
    Base64Png,

    #[serde(rename = "multipart")]
    Multipart,
}

/// Represents an [Image] in a way that it can be serialized and keep references like links instead of only raw data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageDto {
    pub src: ImageSource,
    pub data: String,
}

#[derive(Debug)]
pub enum DtoTransformError {
    Base64Decode,
    Network,
    NoSuchPart,
}

impl ImageDto {
    pub fn to_image(&self, context: &HashMap<String, Image>) -> Result<Image, DtoTransformError> {
        match self.src {
            ImageSource::Base64Png => {
                Image::from_base64(&self.data).map_err(|_| DtoTransformError::Base64Decode)
            }
            ImageSource::Multipart => Ok(context
                .get(&self.data)
                .ok_or(DtoTransformError::NoSuchPart)?
                .clone()),
        }
    }
}
