use serde::{Deserialize, Serialize};

use crate::step::Step;

/// Meta describes all metadata associated with a Moment
#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub timestamp: usize,
    pub user: String,
}

/// A Moment is a atomic piece of editing history, it wraps a single Step
#[derive(Serialize, Deserialize, Debug)]
pub struct Moment {
    pub meta: Meta,
    pub data: Step,
}
