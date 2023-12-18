use std::fmt::{Debug, Display};

use baum::TreeError;
use imagine::DtoTransformError;
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct EngineError {
    user_error: bool,
    reason: String,
}

impl EngineError {
    pub fn user_error(reason: &str) -> Self {
        EngineError {
            user_error: true,
            reason: reason.to_string(),
        }
    }
    pub fn application_error(reason: &str) -> Self {
        EngineError {
            user_error: false,
            reason: reason.to_string(),
        }
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl EngineError {
    #[wasm_bindgen(getter)]
    pub fn is_user_error(&self) -> bool {
        self.user_error
    }

    #[wasm_bindgen(getter)]
    pub fn reason(&self) -> String {
        self.reason.clone()
    }
}

impl Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Error ({}): {}>", &self.reason, &self.user_error)
    }
}

impl From<serde_json::Error> for EngineError {
    fn from(_: serde_json::Error) -> Self {
        EngineError::user_error("Failed serialization")
    }
}

impl From<TreeError> for EngineError {
    fn from(value: TreeError) -> Self {
        match value {
            TreeError::NoSuchNodeUser(idx) => {
                EngineError::user_error(&format!("No such node: {idx}"))
            }
            TreeError::NoSuchNodeInternal(idx) => {
                EngineError::application_error(&format!("No such node: {idx}"))
            }
            TreeError::NoParent(idx) => {
                EngineError::application_error(&format!("No parent of: {idx}"))
            }
            TreeError::CantRemoveRoot => EngineError::user_error("Can't remove root"),
        }
    }
}

impl From<DtoTransformError> for EngineError {
    fn from(_: DtoTransformError) -> Self {
        EngineError::user_error("Bad image DTO")
    }
}

#[cfg(feature = "wasm")]
impl From<serde_wasm_bindgen::Error> for EngineError {
    fn from(_: serde_wasm_bindgen::Error) -> Self {
        EngineError::application_error("Serialization failed")
    }
}

#[cfg(feature = "wasm")]
impl From<wasm_bindgen::prelude::JsError> for EngineError {
    fn from(_: wasm_bindgen::prelude::JsError) -> Self {
        EngineError::user_error("Deserialization failed")
    }
}
