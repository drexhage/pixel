use crate::{tree::TreeError, Tree};

/// The cursor is a helper struct for operating on the tree
/// without having to handle a lot of indices in the application itself.
/// Only the entry node to the cursor has to be kept if it is not the root node.
///
/// # Example
///
/// ```
/// use baum::{Tree,Cursor};
///
/// let mut tree: Tree<i32> = Tree::new(1);
/// let mut cursor = Cursor::new_at_root(&mut tree);
/// cursor.add_child_and_go_down(2);
/// cursor.add_child(4);
/// cursor.add_child(5);
/// cursor.go_up();
/// cursor.add_child_and_go_down(3);
/// cursor.add_child(6);
/// cursor.add_child(7);
/// ```
pub struct Cursor<'a, T> {
    idx: usize,
    tree: &'a mut Tree<T>,
}

impl<'a, T> Cursor<'a, T> {
    /// Creates a new cursor for the given tree at the position of the passed index.
    pub fn new(tree: &'a mut Tree<T>, idx: usize) -> Result<Self, TreeError> {
        // TODO validate tree before returning Ok
        if tree.get_value(idx).is_err() {
            return Err(TreeError::NoSuchNodeInternal(idx));
        }
        Ok(Cursor { idx, tree })
    }

    /// Creates a new cursor for the given tree at the root of the tree.
    pub fn new_at_root(tree: &'a mut Tree<T>) -> Self {
        Cursor {
            idx: tree.get_root(),
            tree,
        }
    }

    /// Retrieves the current index of the cursor position.
    pub fn index(&self) -> usize {
        self.idx
    }

    /// Checks whether or not the cursor is on the root node.
    pub fn is_on_root(&self) -> bool {
        self.idx == self.tree.get_root()
    }

    /// Moves the cursor up to the parent of the current node.
    ///
    /// # Example
    ///
    /// The cursor focus is marked as `(_)`.
    ///
    /// Given the state:
    ///
    /// ```text
    ///    1
    ///   / \
    /// (2)  3
    /// ```
    ///
    /// Then running
    ///
    /// ```no_compile
    /// cursor.go_up();
    /// ```
    ///
    /// Will result in this state:
    ///
    /// ```text
    ///   (1)
    ///   / \
    ///  2   3
    /// ```
    pub fn go_up(&mut self) {
        let root = self.tree.get_root();
        if self.idx != root {
            self.idx = self.tree.get_parent(self.idx).unwrap();
        }
    }

    /// Adds a new child to the current node and moves the cursor down to that node.
    ///
    /// # Example
    ///
    /// The cursor focus is marked as `(_)`.
    ///
    /// Given the state:
    ///
    /// ```text
    ///    1
    ///   / \
    /// (2)  3
    /// ```
    ///
    /// Then running
    ///
    /// ```no_compile
    /// cursor.add_child_and_go_down(4);
    /// ```
    ///
    /// Will result in this state:
    ///
    /// ```text
    ///      1
    ///     / \
    ///    2   3
    ///   /
    /// (4)
    /// ```
    pub fn add_child_and_go_down(&mut self, value: T) -> usize {
        let new_index = self.add_child(value);
        self.idx = new_index;
        new_index
    }

    /// Adds a new child to the current node but keeps the cursor at the current node.
    ///
    /// # Example
    ///
    /// The cursor focus is marked as `(_)`.
    ///
    /// Given the state:
    ///
    /// ```text
    ///    1
    ///   / \
    /// (2)  3
    /// ```
    ///
    /// Then running
    ///
    /// ```no_compile
    /// cursor.add_child(4);
    /// ```
    ///
    /// Will result in this state:
    ///
    /// ```no_rust
    ///      1
    ///     / \
    ///   (2)  3
    ///   /
    ///  4
    /// ```
    pub fn add_child(&mut self, value: T) -> usize {
        self.tree.add_child(self.idx, value).unwrap()
    }

    /// Retrieves the value of the cursor at the current point.
    ///
    /// # Example
    ///
    /// ```
    /// use baum::{Cursor,tr};
    /// let mut tree = tr(1) / tr(2) / tr(3);
    /// let mut cursor = Cursor::new_at_root(&mut tree);
    /// let value = cursor.value();
    /// assert_eq!(*value, 1);
    /// ```
    ///
    /// This simple example showcases the retrival of the root value.
    pub fn value(&self) -> &T {
        return self.tree.get_value(self.idx).unwrap();
    }

    /// Adds a new child to the current node but keeps the cursor at the current node.
    ///
    /// # Example
    ///
    /// The cursor focus is marked as `(_)`.
    ///
    /// Given the state:
    ///
    /// ```text
    ///   (1)
    ///   / \
    ///  2   3
    /// ```
    ///
    /// Then running a `value_mut` that increases the value by 2 will result in:
    ///
    /// ```text
    ///   (3)
    ///   / \
    ///  2   3
    /// ```
    pub fn value_mut(&mut self) -> &mut T {
        return self.tree.value_mut(self.idx).unwrap();
    }

    /// Puts the given value in place of the current position of the cursor.
    pub fn change_value(&mut self, val: T) {
        self.tree.set_value(self.idx, val).unwrap();
    }

    /// Returns the values of all childrens in a vector.
    ///
    /// # Example
    ///
    /// ```
    /// use baum::{Cursor,tr};
    /// let mut tree = tr(1) / tr(2) / tr(3);
    /// let mut cursor = Cursor::new_at_root(&mut tree);
    /// let vals = cursor.children();
    /// assert_eq!(vals.len(), 2);
    /// assert_eq!(*vals[0].0, 2);
    /// assert_eq!(*vals[1].0, 3);
    /// ```
    pub fn children(&'a self) -> Vec<(&'a T, usize)> {
        let children_idx = self.tree.get_children(self.idx).unwrap();
        return children_idx
            .iter()
            .map(|idx| (self.tree.get_value(*idx), *idx))
            .filter_map(|(val, idx)| val.map(|x| (x, idx)).ok())
            .collect();
    }

    /// Destroys the cursor by consuming it and returns the index of the current node of the cursor.
    pub fn destroy(self) -> usize {
        self.idx
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_value_at_root() {
        let mut tree = tr(1) / tr(2) / tr(3);
        let cursor = Cursor::new_at_root(&mut tree);
        let value = cursor.value();
        assert_eq!(*value, 1);
    }

    #[test]
    fn test_go_up() {
        let mut tree = tr(42) / tr(2) / tr(3);
        // hard coded index! Index 1 is in this case tr(2)
        let mut cursor = Cursor::new(&mut tree, 1).unwrap();
        assert_eq!(*cursor.value(), 2);
        cursor.go_up();
        assert_eq!(*cursor.value(), 42);
    }

    #[test]
    fn test_children() {
        let mut tree = tr(1) / tr(12) / tr(32);
        let cursor = Cursor::new_at_root(&mut tree);
        let result = cursor.children();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_is_on_root() {
        let mut tree = tr(1) / tr(12) / tr(32);
        let mut cursor = Cursor::new_at_root(&mut tree);
        assert!(cursor.is_on_root());
        cursor.add_child_and_go_down(23);
        assert!(!cursor.is_on_root());
        cursor.add_child_and_go_down(8);
        assert!(!cursor.is_on_root());
        cursor.go_up();
        assert!(!cursor.is_on_root());
        cursor.go_up();
        assert!(cursor.is_on_root())
    }

    #[test]
    fn test_change_value() {
        let mut tree = tr(1) / tr(12) / tr(32);
        let mut cursor = Cursor::new_at_root(&mut tree);
        cursor.change_value(8);
        cursor.add_child_and_go_down(2);
        cursor.change_value(3);
        assert_eq!(*cursor.value(), 3);
        cursor.go_up();
        assert_eq!(*cursor.value(), 8);
    }
}
