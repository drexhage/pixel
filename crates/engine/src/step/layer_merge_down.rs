use serde::{Deserialize, Serialize};

use crate::{
    layer::{GhostImage, LayerFlag},
    utils, EngineError,
};

use super::IStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerMergeDown {
    id: usize,
}

impl IStep for LayerMergeDown {
    fn perform_on(&self, session: &mut crate::Engine) -> Result<(), EngineError> {
        // find bottom layer id
        let traversal: Vec<usize> = session.content.traverse().into_iter().rev().collect();
        let beneath_idx = *traversal
            .iter()
            .position(|&x| x == self.id)
            .and_then(|x| traversal.get(x + 1))
            .ok_or(EngineError::application_error("No pixel layer beneath"))?;
        // clone top layer
        let top = session
            .content
            .get_value(self.id)
            .map_err(EngineError::from)?
            .clone();
        if top.flag != LayerFlag::Pixel {
            return Ok(());
        }
        // add top layer as ghost to bottom layer and merge, remove top layer
        let bottom = session
            .content
            .value_mut(beneath_idx)
            .map_err(EngineError::from)?;
        if bottom.flag != LayerFlag::Pixel {
            return Ok(());
        }
        let ghost = GhostImage {
            img: top.img,
            mode: top.attr.mode,
            alpha: top.attr.alpha,
        };
        let zombie = bottom.img.clone();
        bottom.ghost = Some(ghost);
        bottom.zombie = Some(zombie);
        utils::merge_ghost(&mut session.blender, &mut session.content, beneath_idx)?;
        utils::remove_layer(&mut session.blender, &mut session.content, self.id)?;
        session.context.idx = Some(beneath_idx);
        Ok(())
    }
}
