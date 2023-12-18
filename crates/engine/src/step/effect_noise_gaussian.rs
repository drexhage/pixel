use baum::Cursor;
use serde::{Deserialize, Serialize};

use crate::{utils, Engine, EngineError};

use super::IStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffectNoiseGaussian {
    id: usize,
    mean: f64,
    stddev: f64,
    seed: u64,
}

impl IStep for EffectNoiseGaussian {
    fn perform_on(&self, session: &mut Engine) -> Result<(), EngineError> {
        let mut cursor = Cursor::new(&mut session.content, self.id).map_err(EngineError::from)?;
        let layer = cursor.value_mut();
        layer.img.gaussian_noise(self.mean, self.stddev, self.seed);
        utils::propagate_changes_up(&mut session.blender, &mut session.content, self.id)?;
        Ok(())
    }
}
