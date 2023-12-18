use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "size[{},{}]", self.width, self.height)
    }
}

impl From<Size> for (u32, u32) {
    fn from(value: Size) -> Self {
        (value.width, value.height)
    }
}

impl From<Size> for (i32, i32) {
    fn from(value: Size) -> (i32, i32) {
        (value.width as i32, value.height as i32)
    }
}

impl From<(u32, u32)> for Size {
    fn from((w, h): (u32, u32)) -> Self {
        Size::new(w, h)
    }
}
