use serde::{Deserialize, Serialize};

use crate::Step;

use super::IStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Compound(Vec<Step>);

impl IStep for Compound {
    fn perform_on(&self, session: &mut crate::Engine) -> Result<(), crate::EngineError> {
        for step in &self.0 {
            step.perform_on(session)?;
        }
        Ok(())
    }
}
