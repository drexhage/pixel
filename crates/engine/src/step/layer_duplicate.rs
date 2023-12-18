use baum::Cursor;
use serde::{Deserialize, Serialize};

use crate::{utils, EngineError};

use super::IStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerDuplicate {
    id: usize,
}

impl IStep for LayerDuplicate {
    fn perform_on(&self, session: &mut crate::Engine) -> Result<(), crate::EngineError> {
        let mut cursor = Cursor::new(&mut session.content, self.id).map_err(EngineError::from)?;
        let layer = cursor.value_mut();
        let mut duplicate = layer.clone();
        let mut name = String::from(&duplicate.name);
        name.push_str(" (2)");
        duplicate.name = name;
        cursor.go_up();
        cursor.add_child(duplicate);
        utils::propagate_changes_up(&mut session.blender, &mut session.content, self.id)?;
        Ok(())
    }
}
