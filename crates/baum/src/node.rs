#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Node<T> {
    pub value: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.parent == other.parent && self.children == other.children
    }
}
