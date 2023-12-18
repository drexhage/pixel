use common::{Color, Position, Size};
use imagine::Image;
use serde::{Deserialize, Serialize};

use crate::{error::EngineError, utils, Engine};

use super::IStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerCreateEmpty {
    pub move_idx: Option<isize>,
    pub size: Option<Size>,
    pub position: Option<Position>,
    pub color: Option<Color>,
    pub name: Option<String>,
}

impl IStep for LayerCreateEmpty {
    fn perform_on(&self, session: &mut Engine) -> Result<(), EngineError> {
        let binding = session.size();
        let size = self.size.unwrap_or(binding);
        let color = self.color.unwrap_or(Color::TRANSPARENT);
        let content = Image::new_from_color(size.width, size.height, &color);
        let idx = utils::add_layer(
            session,
            session.content.root,
            &self.position,
            content,
            self.name.clone(),
        )?;
        let move_idx = self.move_idx.unwrap_or(session.content.root as isize);
        utils::propagate_changes_up(&mut session.blender, &mut session.content, idx)?;
        utils::spawn_layer(session, idx, move_idx)?;
        session.context.idx = Some(idx);
        Ok(())
    }
}
