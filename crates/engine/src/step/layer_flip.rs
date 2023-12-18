use serde::{Deserialize, Serialize};

use crate::{utils, EngineError};

use super::IStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FlipDirection {
    #[serde(rename = "horizontally")]
    Horizontally,

    #[serde(rename = "vertically")]
    Vertically,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerFlip {
    id: usize,
    direction: FlipDirection,
}

impl IStep for LayerFlip {
    fn perform_on(&self, session: &mut crate::Engine) -> Result<(), EngineError> {
        let layer = session
            .content
            .value_mut(self.id)
            .map_err(EngineError::from)?;
        match self.direction {
            FlipDirection::Horizontally => layer.img.flip_horizontally(),
            FlipDirection::Vertically => layer.img.flip_vertically(),
        }
        utils::propagate_changes_up(&mut session.blender, &mut session.content, self.id)?;
        Ok(())
    }
}
