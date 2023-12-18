use baum::Cursor;
use common::Position;
use serde::{Deserialize, Serialize};

use crate::{layer::LayerFlag, utils, Engine, EngineError, Step};

use super::IncrementalStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerMoveRelative {
    pub id: usize,
    pub delta: Position,
}

impl IncrementalStep for LayerMoveRelative {
    type Increment = Position;

    fn start(&self, session: &mut Engine) -> Result<(), EngineError> {
        let mut data = self.clone();
        let mut cursor = Cursor::new(&mut session.content, self.id)?;
        data.delta = Position::zero();
        let layer = cursor.value();
        if layer.flag == LayerFlag::Group {
            return Err(EngineError::application_error("Can't move group"));
        }
        let step = Step::LayerMoveRelative(data);
        session.context.pending_step = Some(step);

        let mut ignore: Option<usize> = None;

        // load images
        while !cursor.is_on_root() {
            cursor.go_up();
            for (layer, child) in cursor.children() {
                if ignore.map(|x| x != child).unwrap_or(true) {
                    session.blender.load(child, &layer.img);
                }
            }
            ignore = Some(cursor.index());
        }
        Ok(())
    }

    fn extend(&self, session: &mut Engine, data: &Self::Increment) -> Result<(), EngineError> {
        if let Some(Step::LayerMoveRelative(mr)) = &mut session.context.pending_step {
            let layer = (session.content)
                .value_mut(self.id)
                .map_err(EngineError::from)?;
            layer.attr.pos += *data;
            mr.delta += *data;
            utils::propagate_changes_up(&mut session.blender, &mut session.content, self.id)?;
            Ok(())
        } else {
            Err(EngineError::user_error(
                "Can't extend witout previous matching",
            ))
        }
    }

    fn finish(&self, session: &mut Engine) -> Result<(), EngineError> {
        session.context.pending_step = None;
        Ok(())
    }

    fn break_up(&self) -> Vec<Self::Increment> {
        vec![self.delta]
    }
}
