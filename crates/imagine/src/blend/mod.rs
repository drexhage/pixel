use serde::{Deserialize, Serialize};

mod blender;
mod software_blender;
#[cfg(feature = "wasm")]
mod webgl_blender;

pub use blender::Blender;
pub use software_blender::SoftwareBlender;
#[cfg(feature = "wasm")]
pub use webgl_blender::WebGlBlender;

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum BlendMode {
    /// source-over
    #[serde(rename = "alpha")]
    Alpha,

    /// destination-out
    #[serde(rename = "remove")]
    Remove,

    /// darken
    #[serde(rename = "darken")]
    Darken,

    /// lighten
    #[serde(rename = "lighten")]
    Lighten,

    /// screen
    #[serde(rename = "screen")]
    Screen,
}

impl BlendMode {
    fn _map_to_canvas(&self) -> String {
        let s = match self {
            BlendMode::Alpha => "source-over",
            BlendMode::Remove => "destination-out",
            BlendMode::Darken => "darken",
            BlendMode::Lighten => "lighten",
            BlendMode::Screen => "screen",
        };
        s.to_string()
    }
}

pub fn generate_blender() -> Box<dyn Blender> {
    let blender: Box<dyn Blender> = Box::new(SoftwareBlender::new());
    #[cfg(feature = "wasm")]
    if let Ok(web_gl_blender) = WebGlBlender::new() {
        return Box::new(web_gl_blender);
    }
    blender
}
