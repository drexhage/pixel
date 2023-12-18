use baum::Cursor;
use common::Position;
use imagine::BlendMode;
use serde::{Deserialize, Serialize};

use crate::{utils, Engine, EngineError};

use super::IStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerAttributes {
    id: usize,
    pos: Option<Position>,
    alpha: Option<f32>,
    mode: Option<BlendMode>,
    visible: Option<bool>,
    name: Option<String>,
}

impl IStep for LayerAttributes {
    fn perform_on(&self, session: &mut Engine) -> Result<(), EngineError> {
        let mut cursor = Cursor::new(&mut session.content, self.id).map_err(EngineError::from)?;
        let layer = cursor.value_mut();
        {
            if let Some(pos) = &self.pos {
                layer.attr.pos = *pos;
            }
            if let Some(alpha) = &self.alpha {
                layer.attr.alpha = *alpha;
            }
            if let Some(mode) = &self.mode {
                layer.attr.mode = *mode;
            }
            if let Some(visible) = &self.visible {
                layer.visible = *visible;
            }
            if let Some(name) = &self.name {
                layer.name = name.clone();
            }
        }
        utils::propagate_changes_up(&mut session.blender, &mut session.content, self.id)?;
        Ok(())
    }
}
