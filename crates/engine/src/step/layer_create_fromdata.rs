use common::Position;
use imagine::ImageDto;
use serde::{Deserialize, Serialize};

use crate::{utils, Engine, EngineError};

use super::IStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerCreateFromData {
    parent: usize,
    img: ImageDto,
    position: Option<Position>,
    name: Option<String>,
}

impl IStep for LayerCreateFromData {
    fn perform_on(&self, session: &mut Engine) -> Result<(), EngineError> {
        let content = self
            .img
            .to_image(&session.context.images)
            .map_err(EngineError::from)?;
        let layer = utils::add_layer(
            session,
            self.parent,
            &self.position,
            content,
            self.name.clone(),
        )?;
        utils::propagate_changes_up(&mut session.blender, &mut session.content, layer)?;
        session.context.idx = Some(layer);
        Ok(())
    }
}
