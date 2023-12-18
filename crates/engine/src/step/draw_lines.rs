use common::{Color, Position, Rectangle};
use imagine::{BlendMode, Image};
use serde::{Deserialize, Serialize};

use crate::{layer::GhostImage, utils, Engine, EngineError, Step};

use super::IncrementalStep;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DrawLine {
    pub id: usize,
    pub radius: f64,
    pub color: Color,
    pub mode: BlendMode,
    pub hardness: f64,
    pub track: Vec<Position>,
    pub distance: usize,
    pub skip: Option<usize>,
}

impl IncrementalStep for DrawLine {
    type Increment = Position;

    fn start(&self, session: &mut Engine) -> Result<(), EngineError> {
        let mut data = self.clone();
        data.track = vec![];
        let step = Step::DrawLine(data);
        let layer = session
            .content
            .value_mut(self.id)
            .map_err(EngineError::from)?;
        let (w, h) = layer.img.size().into();
        let ghost = GhostImage {
            img: Image::new(w, h),
            mode: self.mode,
            alpha: self.color.a as f32 / 255.,
        };
        layer.ghost = Some(ghost);
        layer.zombie = Some(layer.img.clone());
        session.context.pending_step = Some(step);
        Ok(())
    }

    fn extend(&self, session: &mut crate::Engine, data: &Position) -> Result<(), EngineError> {
        let stamp = Image::new_stamp(&self.color, self.hardness, self.radius);

        if let Some(Step::DrawLine(dl)) = &mut session.context.pending_step {
            let root = &session.content.root_value().rectangle();
            let layer = session
                .content
                .value_mut(dl.id)
                .map_err(|_| EngineError::application_error("Invalid ID in pending step"))?;
            // track is in global coordinates
            let track = if let Some(pos) = dl.track.last() {
                if pos == data {
                    return Ok(());
                }
                Position::interpolate(pos, data)
            } else {
                vec![*data, *data]
            };
            dl.track.push(*data);
            let track_len = track.len() - 1; // adjust the fact that interpolate keeps first and last too
            let still_to_skip = dl.skip.unwrap_or(0);
            if still_to_skip >= track_len {
                dl.skip = Some(still_to_skip - track_len);
                return Ok(());
            } else {
                dl.skip = Some(dl.distance - ((track_len - still_to_skip) % dl.distance));
            }
            // track_to_draw is in image coordinates
            let track_to_draw: &Vec<Position> = &track
                .into_iter()
                .skip(still_to_skip)
                .step_by(self.distance)
                .map(|p| p - layer.attr.pos) // maybe??
                .collect();
            if let Some(ghost) = &mut layer.ghost {
                let damage = ghost.img.draw_line(&stamp, track_to_draw); // damage in image coordinates
                let damage = &damage + &layer.attr.pos; // damage in global coordinates
                let damage = Rectangle::intersect(&damage, root); // damage constraint to root area
                utils::propagate_damage(&mut session.blender, &mut session.content, dl.id, &damage)
            } else {
                Err(EngineError::user_error(
                    "Can't call expand without previous step matching up",
                ))
            }
        } else {
            Err(EngineError::user_error(
                "Can't call expand without having initialized",
            ))
        }
    }

    fn finish(&self, session: &mut crate::Engine) -> Result<(), EngineError> {
        utils::merge_ghost(&mut session.blender, &mut session.content, self.id)?;
        utils::propagate_changes_up(&mut session.blender, &mut session.content, self.id)?;
        session.context.pending_step = None;
        Ok(())
    }

    fn break_up(&self) -> Vec<Position> {
        self.track.clone()
    }
}

#[cfg(test)]
mod test {
    use common::{Color, Position};
    use imagine::BlendMode;

    use crate::{
        step::{LayerCreateEmpty, LayerMoveRelative},
        Engine, Step,
    };

    use super::DrawLine;

    #[test]
    fn simple_draw() {
        let color = Color::RED;
        let dl = DrawLine {
            id: 1,
            radius: 10.0,
            color,
            mode: BlendMode::Alpha,
            hardness: 1.0,
            track: vec![(1, 2).into(), (20, 10).into()],
            distance: 5,
            skip: None,
        };
        let cl = LayerCreateEmpty {
            move_idx: None,
            size: None,
            position: None,
            color: None,
            name: None,
        };
        let mut state = Engine::new(100, 100);
        state.perform(&Step::LayerCreateEmpty(cl)).unwrap();
        state.perform(&Step::DrawLine(dl)).unwrap();
        let compare = &state.content.root_value().img.pixel(20, 10);
        assert_eq!(&color, compare);
    }

    #[test]
    fn draw_outside_layer() {
        let color = Color::RED;
        let draw_line = DrawLine {
            id: 1,
            radius: 10.0,
            color,
            mode: BlendMode::Alpha,
            track: vec![
                (-20, -20).into(),
                (1, 2).into(),
                (20, 10).into(),
                (110, 10).into(),
                (112, 10).into(),
            ],
            hardness: 1.0,
            distance: 2,
            skip: None,
        };
        let cl = LayerCreateEmpty {
            move_idx: None,
            size: None,
            position: None,
            color: None,
            name: None,
        };
        let move_layer = LayerMoveRelative {
            id: 1,
            delta: Position::new(10, 10),
        };

        let mut state = Engine::new(100, 100);
        state.perform(&Step::LayerCreateEmpty(cl.clone())).unwrap();
        state.perform(&Step::LayerCreateEmpty(cl)).unwrap();
        state.perform(&Step::LayerMoveRelative(move_layer)).unwrap();
        state.perform(&Step::DrawLine(draw_line)).unwrap();
        let compare = &state.content.root_value().img.pixel(20, 10);
        assert_eq!(&color, compare);
    }
}
