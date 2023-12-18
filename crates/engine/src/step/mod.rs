use common::{Position, Size};
use serde::{Deserialize, Serialize};

mod compound;
mod draw_lines;
mod effect_color_grayscale;
mod effect_noise_gaussian;
mod layer_attributes;
mod layer_create_empty;
mod layer_create_fromdata;
mod layer_create_group;
mod layer_duplicate;
mod layer_flip;
mod layer_merge_down;
mod layer_move;
mod layer_move_relative;
mod layer_remove;

use crate::{error::EngineError, Engine};

pub use self::{
    compound::Compound, draw_lines::DrawLine, effect_noise_gaussian::EffectNoiseGaussian,
    layer_attributes::LayerAttributes, layer_create_empty::LayerCreateEmpty,
    layer_create_fromdata::LayerCreateFromData, layer_create_group::LayerCreateGroup,
    layer_move_relative::LayerMoveRelative, layer_remove::LayerRemove,
};
use self::{
    effect_color_grayscale::EffectColorGrayscale, layer_duplicate::LayerDuplicate,
    layer_flip::LayerFlip, layer_merge_down::LayerMergeDown, layer_move::LayerMove,
};

pub trait IStep {
    /// Perform this step on the given session
    fn perform_on(&self, session: &mut Engine) -> Result<(), EngineError>;

    // Maybe better: Distinguish between procedural and administrative steps (those that perform image manipulation and those that change layers)

    /// Perform this step without doing the actual image processing
    /// This only performs administrative tasks -> keeping track of layers and their properties
    /// Needed for running a verifier that verifies a history without the overhead of performing image processing
    /// Not used right now
    fn perform_without_processing(&self, _session: &mut Engine) -> Result<(), EngineError> {
        Ok(()) // Most steps don't do much
    }
}

/// A StepData is a description of a single atomic manipulation of a LayerState
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Step {
    /// Initializes a new project
    #[serde(rename = "project/create")]
    ProjectCreate { size: Size },

    /// Represents multiple steps that should be performed as one in history
    #[serde(rename = "compound")]
    Compound(Compound),

    /// Creates a new empty layer
    #[serde(rename = "layer/create/empty")]
    LayerCreateEmpty(LayerCreateEmpty),

    /// Creates a new layer given the content of the layer
    #[serde(rename = "layer/create/from_data")]
    LayerCreateFromData(LayerCreateFromData),

    /// Creates a new empty layer group
    #[serde(rename = "layer/create/group")]
    LayerCreateGroup(LayerCreateGroup),

    /// Removes layers by id
    #[serde(rename = "layer/remove")]
    LayerRemove(LayerRemove),

    /// Move layer relative
    #[serde(rename = "layer/move_relative")]
    LayerMoveRelative(LayerMoveRelative),

    /// Move layer relative
    #[serde(rename = "layer/move")]
    LayerMove(LayerMove),

    /// Flip layer
    #[serde(rename = "layer/flip")]
    LayerFlip(LayerFlip),

    /// Merge layer down
    #[serde(rename = "layer/merge_down")]
    LayerMergeDown(LayerMergeDown),

    /// Duplicate a layer
    #[serde(rename = "layer/duplicate")]
    LayerDuplicate(LayerDuplicate),

    /// Set layer attributes
    #[serde(rename = "layer/attr")]
    LayerAttributes(LayerAttributes),

    /// Gaussian noise
    #[serde(rename = "effect/noise/gaussian")]
    EffectNoiseGaussian(EffectNoiseGaussian),

    /// Gaussian noise
    #[serde(rename = "effect/color/grayscale")]
    EffectNoiseGrayscale(EffectColorGrayscale),

    // Lines
    #[serde(rename = "draw/line")]
    DrawLine(DrawLine),
}

/// A IncrementalStep is a way for steps to be incrementally build without having to perform another step.
/// For example when drawing a line, the line starts, extends as it is being drawn and is eventually finished.
/// These actions should be compressed in a single step.
pub trait IncrementalStep {
    type Increment;

    /// Initially starting the step
    fn start(&self, session: &mut Engine) -> Result<(), EngineError>;

    /// Incrementing the step with a new information
    fn extend(&self, session: &mut Engine, data: &Self::Increment) -> Result<(), EngineError>;

    /// Finishing the step
    fn finish(&self, session: &mut Engine) -> Result<(), EngineError>;

    /// Given a finished step, break it up into single increments such that it can be performed with the above methods
    fn break_up(&self) -> Vec<Self::Increment>;
}

/// Every ExtendableStep is also a normal step
impl<T> IStep for T
where
    T: IncrementalStep,
{
    fn perform_on(&self, session: &mut Engine) -> Result<(), EngineError> {
        log::info!("Perform incremental step as unit");
        self.start(session)?;
        for increment in self.break_up() {
            self.extend(session, &increment)?;
        }
        self.finish(session)?;
        Ok(())
    }
}

// The ugly
impl Step {
    pub fn as_step(&self) -> Box<&dyn IStep> {
        match self {
            Step::LayerCreateEmpty(s) => Box::new(s),
            Step::LayerCreateFromData(s) => Box::new(s),
            Step::LayerRemove(s) => Box::new(s),
            Step::LayerMove(s) => Box::new(s),
            Step::LayerAttributes(s) => Box::new(s),
            Step::EffectNoiseGaussian(s) => Box::new(s),
            Step::DrawLine(s) => Box::new(s),
            Step::Compound(s) => Box::new(s),
            Step::LayerCreateGroup(s) => Box::new(s),
            Step::LayerMoveRelative(s) => Box::new(s),
            Step::EffectNoiseGrayscale(s) => Box::new(s),
            Step::LayerFlip(s) => Box::new(s),
            Step::LayerMergeDown(s) => Box::new(s),
            Step::LayerDuplicate(s) => Box::new(s),
            Step::ProjectCreate { size: _size } => panic!(),
        }
    }

    pub fn perform_on(&self, session: &mut Engine) -> Result<(), EngineError> {
        return self.as_step().perform_on(session);
    }

    pub fn as_extendable(&self) -> Option<Box<dyn IncrementalStep<Increment = Position>>> {
        match self {
            Step::DrawLine(s) => Some(Box::new(s.clone())),
            Step::LayerMoveRelative(s) => Some(Box::new(s.clone())),
            _ => None,
        }
    }

    pub fn log_debug(&self, message: &str) {
        if log::log_enabled!(log::Level::Debug) {
            let json = serde_json::to_string(&self)
                .map_err(EngineError::from)
                .expect("Failed to unwrap");
            log::debug!("{}: {}", message, &json);
        }
    }
}
