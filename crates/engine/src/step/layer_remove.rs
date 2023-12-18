use serde::{Deserialize, Serialize};

use crate::{error::EngineError, utils, Engine};

use super::IStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerRemove {
    ids: Vec<usize>,
}

impl IStep for LayerRemove {
    fn perform_on(&self, session: &mut Engine) -> Result<(), EngineError> {
        for id in &self.ids {
            let parent = session.content.get_parent(*id).map_err(EngineError::from)?;
            session
                .content
                .remove_entry(*id)
                .map_err(EngineError::from)?;
            utils::propagate_changes_up(&mut session.blender, &mut session.content, parent)?;
        }
        Ok(())
    }
}
