use baum::Cursor;
use common::Position;
use imagine::{BlendMode, Image};
use serde::{Deserialize, Serialize};

use crate::{
    layer::{Layer, LayerAttributes, LayerFlag},
    utils, Engine, EngineError,
};

use super::IStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerCreateGroup {
    move_idx: Option<isize>,
}

impl IStep for LayerCreateGroup {
    fn perform_on(&self, session: &mut Engine) -> Result<(), EngineError> {
        let size = session.size();
        let layer = Layer {
            img: Image::new(size.width, size.height),
            ghost: None,
            zombie: None,
            attr: LayerAttributes {
                pos: Position::new(0, 0),
                mode: BlendMode::Alpha,
                alpha: 1.0,
            },
            flag: LayerFlag::Group,
            visible: true,
            name: "".to_string(),
        };
        let move_idx = self.move_idx.unwrap_or(session.content.root as isize);
        let root_idx = session.content.root;
        let mut cursor = Cursor::new(&mut session.content, root_idx).map_err(EngineError::from)?;
        cursor.add_child_and_go_down(layer);
        let idx = cursor.index();
        let layer = cursor.value_mut();
        layer.name = format!("Group #{}", idx).to_string();
        utils::move_layer(session, idx, move_idx)?;
        session.context.idx = Some(idx);
        Ok(())
    }
}
