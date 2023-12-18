use baum::Cursor;

use crate::{Engine, EngineError, Step};

impl Engine {
    pub fn start_step(&mut self, step: &Step) -> Result<Option<usize>, EngineError> {
        let ext = step
            .as_extendable()
            .ok_or(EngineError::user_error("Can't do that"))?;
        ext.start(self)?;
        Ok(None)
    }

    pub fn extend_step(&mut self, x: f64, y: f64) -> Result<Option<usize>, EngineError> {
        let ps = &self.context.pending_step.clone();
        let ext = ps
            .clone()
            .and_then(|x| x.as_extendable())
            .ok_or(EngineError::user_error("Can't extend without starting"))?;
        ext.extend(self, &(x as i32, y as i32).into())?;
        Ok(None)
    }

    pub fn finish_step(&mut self) -> Result<Option<usize>, EngineError> {
        let ps = &self
            .context
            .pending_step
            .clone()
            .ok_or(EngineError::user_error("Can't finish without starting"))?;
        let ext = ps
            .as_extendable()
            .ok_or(EngineError::user_error("Can't finish without starting"))?;
        self.current = self.push_moment(ps)?;
        self.context.pending_step = None;
        ext.finish(self)?;
        self.blender.clean();
        if log::log_enabled!(log::Level::Debug) {
            let cursor = Cursor::new(&mut self.history, self.current).map_err(EngineError::from)?;
            let step = &cursor.value().data;
            let json = serde_json::to_string(&step).map_err(EngineError::from)?;
            log::debug!("{}", &json);
        }
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use crate::{step::LayerCreateEmpty, Engine, Step};

    #[test]
    fn test() {
        // should just not error out
        let mut state = Engine::new(100, 100);
        state
            .perform(&Step::LayerCreateEmpty(LayerCreateEmpty {
                move_idx: None,
                size: None,
                position: None,
                color: None,
                name: None,
            }))
            .expect("Failed to create empty layer");
    }
}
