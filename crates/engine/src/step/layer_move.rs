use serde::{Deserialize, Serialize};

use crate::{utils, EngineError};

use super::IStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerMove {
    id: usize,
    move_idx: isize,
}

impl IStep for LayerMove {
    fn perform_on(&self, session: &mut crate::Engine) -> Result<(), crate::EngineError> {
        let parent_idx = session
            .content
            .get_parent(self.id)
            .map_err(EngineError::from)?;
        utils::move_layer(session, self.id, self.move_idx)?;
        utils::propagate_changes_up(&mut session.blender, &mut session.content, self.id)?;
        utils::propagate_changes_up(&mut session.blender, &mut session.content, parent_idx)?;
        Ok(())
    }
}
