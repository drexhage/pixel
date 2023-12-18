use serde::{Deserialize, Serialize};

use crate::{utils, EngineError};

use super::IStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffectColorGrayscale {
    pub id: usize,
}

impl IStep for EffectColorGrayscale {
    fn perform_on(&self, session: &mut crate::Engine) -> Result<(), crate::EngineError> {
        (session.content)
            .value_mut(self.id)
            .map_err(EngineError::from)?
            .img
            .grayscale();
        utils::propagate_changes_up(&mut session.blender, &mut session.content, self.id)?;
        Ok(())
    }
}
