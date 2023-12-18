use std::collections::HashMap;

use baum::{Cursor, Tree};
use common::{Position, Size};
#[cfg(feature = "wasm")]
use imagine::WebGlBlender;
use imagine::{generate_blender, Blender, Image, SoftwareBlender};
use serde::Serialize;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    error::EngineError,
    layer::{Layer, LayerFlag},
    moment::{Meta, Moment},
    step::Step,
    utils,
};

/// `Engine` keeps the whole state of a image editing session.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Serialize)]
pub struct Engine {
    /// The name of the session
    pub(crate) name: String,

    /// Keep the version in the state in case for future changes in the architecture
    pub(crate) version: String,

    /// Editing history
    pub(crate) history: Tree<Moment>,

    /// The current active point in history, as referenced by the tree node id
    pub(crate) current: usize,

    /// Undo stack
    pub(crate) redo_stack: Vec<usize>,

    /// The content as it is created when wandering from the root to the current point in history
    pub(crate) content: Tree<Layer>,

    /// Context (needed for rendering multiform api calls)
    pub(crate) context: EngineContext,

    /// Blender
    #[serde(skip)]
    pub(crate) blender: Box<dyn Blender>,
}

#[derive(Serialize)]
pub struct EngineContext {
    pub(crate) images: HashMap<String, Image>,
    pub(crate) pending_step: Option<Step>,
    pub(crate) idx: Option<usize>,
}

///
/// The WASM-facing interface goes here
///
// #[cfg(target_family = "wasm")]
// #[wasm_bindgen]
impl Engine {
    // #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Engine {
        log::info!("Initializing session");
        let size = Size { width, height };
        let step = Step::ProjectCreate { size };
        let init_moment = Moment {
            meta: Meta {
                timestamp: 87,
                user: "default".to_string(),
            },
            data: step,
        };
        let mut root_layer = Layer::default(width, height);
        root_layer.flag = LayerFlag::Root;
        let context = EngineContext {
            images: HashMap::new(),
            pending_step: None,
            idx: None,
        };
        let blender = generate_blender();
        Engine {
            name: "default".to_string(),
            version: "v1".to_string(),
            content: Tree::new(root_layer),
            history: Tree::new(init_moment),
            redo_stack: vec![],
            current: 0,
            context,
            blender,
        }
    }

    pub fn switch_blender(&mut self, value: &str) -> Result<(), EngineError> {
        self.blender = match value {
            "Software" => Box::new(SoftwareBlender::new()),
            #[cfg(feature = "wasm")]
            "WebGL" => Box::new(WebGlBlender::new().map_err(|e| {
                EngineError::application_error(&format!("Failed to create WebGL blenderer: {}", e))
            })?),
            _ => panic!("No such blender"),
        };
        log::info!("Changed blender to '{}'", value);
        Ok(())
    }

    pub fn content_as_base64(&self) -> String {
        return self.content.root_value().img.encode_base64().unwrap();
    }

    pub fn content_as_png_bytes(&self) -> Vec<u8> {
        return self.content.root_value().img.into_png_bytes();
    }

    pub fn size(&self) -> Size {
        self.content.root_value().img.size()
    }

    pub fn undoable(&self) -> bool {
        self.current != self.history.root
    }

    pub fn redoable(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn bytes(&self) -> Vec<u8> {
        return self.content.root_value().img.into_bytes();
    }

    pub fn reconstruct(
        steps: &[Step],
        context: HashMap<String, Image>,
    ) -> Result<Engine, EngineError> {
        let first = steps
            .first()
            .ok_or(EngineError::user_error("No step provided"))?;
        if let Step::ProjectCreate { size } = first {
            let mut result = Engine::new(size.width, size.height);
            let context = EngineContext {
                images: context,
                pending_step: None,
                idx: None,
            };
            result.context = context;
            for step in steps.iter().skip(1) {
                result.perform(step)?;
            }
            Ok(result)
        } else {
            // first step always has to be project/create
            Err(EngineError::user_error(
                "Create project has to be first step",
            ))
        }
    }

    pub fn set_context_entry(&mut self, lkj: String, l: Image) {
        self.context.images.insert(lkj, l);
    }

    pub fn perform(&mut self, step: &Step) -> Result<Option<usize>, EngineError> {
        if log::log_enabled!(log::Level::Debug) {
            log::debug!("Performing: {}", log::as_serde!(&step));
        }
        self.context.idx = None;
        step.perform_on(self)?;
        self.current = self.push_moment(step)?;
        self.redo_stack = vec![];
        let result = self.context.idx;

        Ok(result)
    }

    pub fn push_moment(&mut self, step: &Step) -> Result<usize, EngineError> {
        let meta = Meta {
            timestamp: 87,
            user: String::from("default"),
        };
        let moment = Moment {
            data: step.clone(),
            meta,
        };
        let mut cursor = Cursor::new(&mut self.history, self.current).map_err(EngineError::from)?;
        cursor.add_child_and_go_down(moment);
        let idx = cursor.destroy();
        Ok(idx)
    }

    pub fn first_hit_layer(&self, x: i32, y: i32) -> Option<usize> {
        let g: Vec<usize> = self.content.traverse().into_iter().rev().collect();
        let pos = Position::new(x, y);
        for idx in g {
            let a = self
                .content
                .get_value(idx)
                .expect("Internal issues with layer traversal");
            if a.flag == LayerFlag::Pixel && a.is_hit(&pos) {
                return Some(idx);
            }
        }
        None
    }

    pub fn undo(&mut self) -> Result<(), EngineError> {
        if !self.undoable() {
            return Err(EngineError::user_error(
                "Can't undo without anything to undo",
            ));
        }
        self.redo_stack.push(self.current);
        let parent_idx = self.history.get_parent(self.current).map_err(|e| {
            self.redo_stack.pop();
            e
        })?;
        log::debug!("current {} -> {}", self.current, parent_idx);
        self.current = parent_idx;
        let mut temp_idx = parent_idx;
        let mut path: Vec<usize> = vec![temp_idx];
        while temp_idx != self.history.root {
            temp_idx = self.history.get_parent(temp_idx)?;
            path.push(temp_idx);
        }
        path.reverse();
        let mut initalized = false;
        for idx in path {
            log::debug!("Redo {idx}");
            let step = {
                let cursor = Cursor::new(&mut self.history, idx)?;
                &cursor.value().data.clone()
            };
            if let Step::ProjectCreate { size } = step {
                let mut root_layer = Layer::default(size.width, size.height);
                root_layer.flag = LayerFlag::Root;
                self.content = Tree::new(root_layer);
                initalized = true;
            } else {
                if !initalized {
                    return Err(EngineError::application_error("Uninitialized undoing"));
                }
                step.log_debug("Redoing");
                step.perform_on(self)?;
            }
        }
        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), EngineError> {
        let idx = self
            .redo_stack
            .pop()
            .ok_or(EngineError::user_error("Nothing to redo"))?;
        if self.history.get_parent(idx)? != self.current {
            return Err(EngineError::application_error("Inconsistent redo"));
        }
        let step = {
            let cursor = Cursor::new(&mut self.history, idx)?;
            &cursor.value().data.clone()
        };
        self.current = idx;
        step.perform_on(self)
    }

    pub fn log_history(&self, message: &str) {
        if log::log_enabled!(log::Level::Debug) {
            let json = serde_json::to_string(&self.history)
                .map_err(EngineError::from)
                .expect("Failed to unwrap");
            log::debug!("{}: {}", message, &json);
        }
    }

    pub fn move_layer_up(&mut self, idx: usize) -> Result<(), EngineError> {
        let parent_idx = self.content.get_parent(idx)?;
        let siblings = self.content.get_children(parent_idx)?;
        let list_idx = siblings
            .iter()
            .position(|x| *x == idx)
            .ok_or(EngineError::user_error("No such node"))?;
        if let Some(neighbor_idx) = self.content.nodes[parent_idx].children.get(list_idx + 1) {
            // right neighbor exists -> either go into right neighbor if its a group or above right neighbor else
            let right_neighbor = self
                .content
                .nodes
                .get(*neighbor_idx)
                .ok_or(EngineError::application_error("Inconsistent"))?;
            if right_neighbor.value.flag == LayerFlag::Group {
                let move_idx = right_neighbor
                    .children
                    .first()
                    .map(|x| *x as isize)
                    .unwrap_or(-(right_neighbor.id as isize));
                utils::move_layer(self, idx, move_idx)?;
            } else {
                // move index is either the right neighbor of the right neighbor or minus the parent
                let move_idx = self.content.nodes[parent_idx]
                    .children
                    .get(list_idx + 2)
                    .map(|x| *x as isize)
                    .unwrap_or(-(parent_idx as isize));
                utils::move_layer(self, idx, move_idx)?;
            }
        } else {
            // no right neighbor exists -> try to escape group if group is not root
            if parent_idx == self.content.root {
                return Ok(());
            }
            let grandparent_idx = self.content.get_parent(parent_idx)?;
            let parent_siblings = self.content.get_children(grandparent_idx)?;
            let parent_list_idx = parent_siblings
                .iter()
                .position(|x| *x == parent_idx)
                .ok_or(EngineError::user_error("No such node"))?;
            let move_idx = self.content.nodes[grandparent_idx]
                .children
                .get(parent_list_idx + 1)
                .map(|x| *x as isize)
                .unwrap_or(-(grandparent_idx as isize));
            utils::move_layer(self, idx, move_idx)?;
        }
        utils::propagate_changes_up(&mut self.blender, &mut self.content, idx)?;
        utils::propagate_changes_up(&mut self.blender, &mut self.content, parent_idx)?;
        Ok(())
    }

    pub fn move_layer_down(&mut self, idx: usize) -> Result<(), EngineError> {
        let parent_idx = self.content.get_parent(idx)?;
        let siblings = self.content.get_children(parent_idx)?;
        let list_idx = siblings
            .iter()
            .position(|x| *x == idx)
            .ok_or(EngineError::user_error("No such node"))?;
        if let Some(neighbor_idx) = self.content.nodes[parent_idx].children.get(list_idx - 1) {
            // left neighbor exists -> either go into left neighbor if its a group or below left neighbor else
            let left_neighbor = self
                .content
                .nodes
                .get(*neighbor_idx)
                .ok_or(EngineError::application_error("Inconsistent"))?;
            let move_idx: isize = if left_neighbor.value.flag == LayerFlag::Group {
                -(left_neighbor.id as isize)
            } else {
                left_neighbor.id as isize
            };
            utils::move_layer(self, idx, move_idx)?;
        } else {
            // no left neighbor exists -> try to escape group if group is not root
            if parent_idx == self.content.root {
                return Ok(());
            }
            let grandparent_idx = self.content.get_parent(parent_idx)?;
            let parent_siblings = self.content.get_children(grandparent_idx)?;
            let parent_list_idx = parent_siblings
                .iter()
                .position(|x| *x == parent_idx)
                .ok_or(EngineError::user_error("No such node"))?;
            let move_idx = self.content.nodes[grandparent_idx]
                .children
                .get(parent_list_idx)
                .map(|x| *x as isize)
                .unwrap_or(grandparent_idx as isize);
            utils::move_layer(self, idx, move_idx)?;
        }
        utils::propagate_changes_up(&mut self.blender, &mut self.content, idx)?;
        utils::propagate_changes_up(&mut self.blender, &mut self.content, parent_idx)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::EngineError, step::Step};

    use super::Engine;

    const PNG_BASE64: &str = "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";
    const LAYER_CREATE_EMPTY: &str = r#"
        {
            "type": "layer/create/empty",
            "parent": 0,
            "size": {"width": 100, "height": 100},
            "position": null
        }
    "#;

    #[test]
    fn letsgo() -> Result<(), EngineError> {
        let mut state = Engine::new(100, 100);
        let step: Step = serde_json::from_str(LAYER_CREATE_EMPTY).unwrap();
        state.perform(&step)?;
        Ok(())
    }

    #[test]
    fn create_and_delete_layer() -> Result<(), EngineError> {
        let mut state = Engine::new(100, 100);
        let step = serde_json::from_str(LAYER_CREATE_EMPTY).unwrap();
        state.perform(&step)?;
        assert_eq!(state.content.get_children(0).unwrap().len(), 1);
        let layer_remove = r#"{"type":"layer/remove","ids":[1]}"#;
        let step = serde_json::from_str(layer_remove).unwrap();
        state.perform(&step)?;
        assert_eq!(state.content.get_children(0).unwrap().len(), 0);
        Ok(())
    }

    #[test]
    fn illegal_base64_in_step_should_fail() -> Result<(), EngineError> {
        let mut state = Engine::new(100, 100);
        let create_layer = r#"{
            "type": "layer/create/from_data",
            "parent": 0,
            "img": {
                "src":"encode/png",
                "data":"not deserializable"
            }
        }"#;
        let step = serde_json::from_str(create_layer).unwrap();
        assert!(state.perform(&step).is_err());
        Ok(())
    }

    #[test]
    fn render_something_sometimes() -> Result<(), EngineError> {
        let mut state = Engine::new(100, 100);
        let create_layer = r#"{"type": "layer/create/empty", "parent":0}"#;
        let step = serde_json::from_str(create_layer).unwrap();
        state.perform(&step)?;
        let create_layer_from_data = r#"{
            "type": "layer/create/from_data",
            "parent": 0,
            "img": {
                "src":"encode/png",
                "data":"DATA"
            }
        }"#
        .replace("DATA", PNG_BASE64);
        let step = serde_json::from_str(&create_layer_from_data).unwrap();
        state.perform(&step)?;

        Ok(())
    }
}
