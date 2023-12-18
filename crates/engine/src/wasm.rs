use common::Size;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{Engine, EngineError, Step};

#[wasm_bindgen(start)]
fn start() {
    //std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Trace).expect("Failed to init logging");
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn _new(width: u32, height: u32) -> Engine {
        Self::new(width, height)
    }

    pub fn perform_step(&mut self, val: JsValue) -> Result<Option<usize>, EngineError> {
        let step: Step = serde_wasm_bindgen::from_value(val).map_err(EngineError::from)?;
        self.perform(&step)?;
        Ok(None)
    }

    #[no_mangle]
    #[wasm_bindgen(getter)]
    pub fn pointer(&mut self) -> u32 {
        let root = self.content.root_value();
        let ptr = root.img.into_array().as_ptr();
        log::debug!("Accessing content pointer");
        ptr as u32
    }

    #[wasm_bindgen(js_name = start_step)]
    pub fn _start_step(&mut self, val: JsValue) -> Result<Option<usize>, EngineError> {
        let step: Step = serde_wasm_bindgen::from_value(val).map_err(EngineError::from)?;
        self.start_step(&step)
    }

    #[wasm_bindgen(js_name = extend_step)]
    pub fn _extend_step(&mut self, x: f64, y: f64) -> Result<Option<usize>, EngineError> {
        self.extend_step(x, y)
    }

    #[wasm_bindgen(js_name = finish_step)]
    pub fn _finish_step(&mut self) -> Result<Option<usize>, EngineError> {
        self.finish_step()
    }

    #[wasm_bindgen(js_name = undo)]
    pub fn _undo(&mut self) -> Result<(), EngineError> {
        self.undo()
    }

    #[wasm_bindgen(js_name = redo)]
    pub fn _redo(&mut self) -> Result<(), EngineError> {
        self.redo()
    }

    #[wasm_bindgen(js_name = get_first_hit)]
    pub fn _first_hit_layer(&self, x: i32, y: i32) -> Option<usize> {
        self.first_hit_layer(x, y)
    }

    #[wasm_bindgen(js_name = move_layer_up)]
    pub fn _move_layer_up(&mut self, idx: usize) -> Result<(), EngineError> {
        self.move_layer_up(idx)
    }

    #[wasm_bindgen(js_name = move_layer_down)]
    pub fn _move_layer_down(&mut self, idx: usize) -> Result<(), EngineError> {
        self.move_layer_down(idx)
    }

    #[wasm_bindgen(js_name = switch_blender)]
    pub fn _switch_blender(&mut self, value: &str) -> Result<(), EngineError> {
        self.switch_blender(value)
    }

    #[wasm_bindgen(getter)]
    pub fn current(&self) -> usize {
        self.current
    }

    #[wasm_bindgen(getter)]
    pub fn context_idx(&self) -> Option<usize> {
        self.context.idx
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Result<JsValue, EngineError> {
        serde_wasm_bindgen::to_value(&self.content).map_err(EngineError::from)
    }

    #[wasm_bindgen(getter)]
    pub fn history(&self) -> Result<JsValue, EngineError> {
        serde_wasm_bindgen::to_value(&self.history).map_err(EngineError::from)
    }

    #[wasm_bindgen(getter, js_name = undoable)]
    pub fn _undoable(&self) -> bool {
        self.current != self.history.root
    }

    #[wasm_bindgen(getter, js_name = redoable)]
    pub fn _redoable(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    #[wasm_bindgen(getter)]
    pub fn version(&self) -> String {
        self.version.clone()
    }

    #[wasm_bindgen(getter, js_name = size)]
    pub fn _size(&self) -> Size {
        self.content.root_value().img.size()
    }

    #[wasm_bindgen(getter)]
    pub fn blender(&self) -> String {
        self.blender.name().to_string()
    }
}
