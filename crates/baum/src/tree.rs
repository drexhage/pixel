use std::cmp::Ordering;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum TreeError {
    NoSuchNodeInternal(usize),
    NoSuchNodeUser(usize),
    NoParent(usize),
    CantRemoveRoot,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Node<T> {
    pub id: usize,
    pub value: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T> Node<T> {
    pub fn new(id: usize, value: T, parent: Option<usize>) -> Self {
        Node {
            id,
            value,
            parent,
            children: vec![],
        }
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.parent == other.parent && self.children == other.children
    }
}

/// Tree structure
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Tree<T> {
    pub nodes: Vec<Node<T>>,
    pub root: usize,
}

impl<T> Tree<T> {
    /// A tree constructor. A tree can never be fully empty (use Option<Tree> for that instead).
    /// Thus initializing a tree also comes with passing the inital root value:
    ///
    /// # Example
    ///
    /// ```no_compile
    /// let tree: Tree<i32> = Tree::new(42);
    /// ```
    pub fn new(inital: T) -> Self {
        Tree {
            nodes: vec![Node::new(0, inital, None)],
            root: 0,
        }
    }

    pub fn root_value(&self) -> &T {
        return &self
            .nodes
            .get(self.root)
            .expect("No root value found")
            .value;
    }

    /// Retrieves the index of the root node of the tree.
    pub fn get_root(&self) -> usize {
        self.root
    }

    /// Adds a child with the passed value to the parent node with the given index.
    ///
    /// # Example
    ///
    /// This code snippet:
    ///
    /// ```
    /// use baum::{Tree,Cursor};
    ///
    /// let mut tree = Tree::new(1);
    /// tree.add_child(tree.get_root(), 2);
    /// tree.add_child(tree.get_root(), 3);
    /// ```
    ///
    /// creates a tree of the following structure:
    ///
    /// ```no_rust
    ///   1
    ///  / \
    /// 2   3
    /// ```
    pub fn add_child(&mut self, parent: usize, value: T) -> Result<usize, TreeError> {
        let node = Node::new(self.nodes.len(), value, Some(parent));
        self.nodes.push(node);
        let new_idx = self.nodes.len() - 1;
        let node = self
            .nodes
            .get_mut(parent)
            .ok_or(TreeError::NoSuchNodeUser(parent))?;
        node.children.push(new_idx);
        Ok(new_idx)
    }

    /// Removes a child of the given index.
    /// Since any node in a tree can only have one parent, it is not necessary to also pass the parent node index.
    pub fn remove_entry(&mut self, idx: usize) -> Result<(), TreeError> {
        let node: &mut Node<T> = self
            .nodes
            .get_mut(idx)
            .ok_or(TreeError::NoSuchNodeUser(idx))?;
        let parent_idx = node.parent.ok_or(TreeError::CantRemoveRoot)?;
        let parent: &mut Node<T> = (self.nodes)
            .get_mut(parent_idx)
            .ok_or(TreeError::NoSuchNodeInternal(parent_idx))?;
        parent.children.retain(|x| *x != idx);
        Ok(())
    }

    /// Traverses the tree.
    ///
    /// # Example
    ///
    /// Given the tree:
    ///
    /// ```no_rust
    ///         0
    ///       /   \
    ///      1     2
    ///     / \   / \
    ///    3   4 5   6
    /// ```
    ///
    /// Produces the traversal: `0 1 3 4 2 5 6`
    pub fn traverse(&self) -> Vec<usize> {
        let mut result = vec![];
        let mut stack = vec![self.root];
        while let Some(current) = stack.pop() {
            let node = self.nodes.get(current).unwrap();
            let mut children = node.children.clone();
            result.push(current);
            children.reverse();
            stack.append(&mut children);
        }
        result
    }

    /// Moves the node at given index to the spot defined by the given move index.
    /// Either updates the position or returns an error for bad indexes.
    ///
    /// # Move indexes
    ///
    /// Move indexes can be determined by the layer setup like this:
    /// - The first spot has move index 0
    /// - Given item with ID x, the spot beneath this item has move index x
    /// - Given a group with ID x, the first spot in this group has move index -x
    ///
    /// # Example
    ///
    /// Given the tree:
    ///
    /// ```no_compile
    ///         0
    ///       /   \
    ///      1     2
    ///     /
    ///    3
    /// ```
    ///
    /// Then move indexes come like this (with the numbers in [] being the move indexes):
    ///
    /// ```no_compile
    ///          0
    ///        /   \
    ///    [0]1[1] 2[2]
    ///      /
    /// [-1]3[3]
    /// ```
    ///
    /// If we insert the value 4 at move_idx 1 then the tree becomes
    ///
    /// ```no_compile
    ///          0
    ///        / | \
    ///       1  4  2
    ///      /
    ///     3
    /// ```
    pub fn move_node(&mut self, idx: usize, move_idx: isize) -> Result<(), TreeError> {
        if idx as isize == move_idx {
            return Ok(());
        }

        // remove from the current parent
        let node = self
            .nodes
            .get_mut(idx)
            .ok_or(TreeError::NoSuchNodeUser(idx))?;
        if let Some(parent_idx) = node.parent {
            let parent = self
                .nodes
                .get_mut(parent_idx)
                .ok_or(TreeError::NoSuchNodeInternal(parent_idx))?;
            parent.children.retain(|&x| x != idx);
        }

        // put into the new spot
        match move_idx.cmp(&0) {
            Ordering::Less => {
                let reference_idx = -move_idx as usize;
                // update parent of node
                let node = self
                    .nodes
                    .get_mut(idx)
                    .ok_or(TreeError::NoSuchNodeInternal(idx))?;
                node.parent = Some(reference_idx);
                // add new first child to reference point
                let reference = self
                    .nodes
                    .get_mut(reference_idx)
                    .ok_or(TreeError::NoSuchNodeInternal(reference_idx))?;
                reference.children.push(idx);
                Ok(())
            }
            Ordering::Equal => {
                let node = self
                    .nodes
                    .get_mut(idx)
                    .ok_or(TreeError::NoSuchNodeInternal(idx))?;
                node.parent = Some(self.root);
                let root = self
                    .nodes
                    .get_mut(self.root)
                    .ok_or(TreeError::NoSuchNodeInternal(self.root))?;
                root.children.push(idx);
                Ok(())
            }
            Ordering::Greater => {
                let id = move_idx as usize;
                let reference = self
                    .nodes
                    .get(id)
                    .ok_or(TreeError::NoSuchNodeInternal(id))?
                    .parent;
                // get parent of reference point
                match reference {
                    Some(parent_idx) => {
                        // update parent of node
                        let node = self
                            .nodes
                            .get_mut(idx)
                            .ok_or(TreeError::NoSuchNodeInternal(idx))?;
                        node.parent = Some(parent_idx);
                        let parent = self
                            .nodes
                            .get_mut(parent_idx)
                            .ok_or(TreeError::NoSuchNodeInternal(parent_idx))?;
                        let pos = parent.children.iter().position(|&x| x == id).unwrap();
                        parent.children.insert(pos, idx);
                        // parent.children.push(idx);
                        Ok(())
                    }
                    None => Err(TreeError::NoParent(id)),
                }
            }
        }
    }

    /// Returns all indices of the children of the provided node index.
    pub fn get_children(&self, idx: usize) -> Result<Vec<usize>, TreeError> {
        let node = self.nodes.get(idx).ok_or(TreeError::NoSuchNodeUser(idx))?;
        Ok(node.children.to_vec())
    }

    /// Returns the parent node index of the given node index.
    pub fn get_parent(&self, idx: usize) -> Result<usize, TreeError> {
        let node = self.nodes.get(idx).ok_or(TreeError::NoSuchNodeUser(idx))?;
        node.parent.ok_or(TreeError::NoParent(idx))
    }

    /// Returns the value of the given node index.
    pub fn get_value(&self, idx: usize) -> Result<&T, TreeError> {
        let node = self.nodes.get(idx).ok_or(TreeError::NoSuchNodeUser(idx))?;
        Ok(&node.value)
    }

    pub fn value_mut(&mut self, idx: usize) -> Result<&mut T, TreeError> {
        let node = self
            .nodes
            .get_mut(idx)
            .ok_or(TreeError::NoSuchNodeUser(idx))?;
        Ok(&mut node.value)
    }

    /// Sets the value of the given node index.
    pub fn set_value(&mut self, idx: usize, val: T) -> Result<(), TreeError> {
        let node = self
            .nodes
            .get_mut(idx)
            .ok_or(TreeError::NoSuchNodeUser(idx))?;
        node.value = val;
        Ok(())
    }

    /// Adds a whole other tree as a child to the node provided as an index.
    pub fn add_tree_as_child(&mut self, other: Tree<T>, idx: usize) -> Result<(), TreeError> {
        let offset = self.nodes.len();
        for mut node in other.nodes {
            // adjust parent to new index
            node.parent = if let Some(parent_idx) = node.parent {
                Some(parent_idx + offset)
            } else {
                Some(idx) // the root of the passed tree is going to get a parent: the passed index
            };
            // adjust children to new index
            node.children = node.children.into_iter().map(|x| x + offset).collect();
            // update own id
            node.id += offset;
            // add node to arena
            self.nodes.push(node);
        }
        // add added node as a child
        let g = self
            .nodes
            .get_mut(idx)
            .ok_or(TreeError::NoSuchNodeUser(idx))?;
        g.children.push(other.root + offset);
        Ok(())
    }
}

impl<T: PartialEq> PartialEq for Tree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes && self.root == other.root
    }
}

#[cfg(test)]
mod tests {
    use crate::tr;

    use super::*;

    #[test]
    fn basic_traversal() {
        let tree = tr(0) / (tr(1) / tr(2) / tr(3)) / (tr(4) / tr(5) / tr(6));
        let exptected = vec![0, 1, 2, 3, 4, 5, 6];
        assert_eq!(exptected, tree.traverse());
    }

    #[test]
    fn add_children() {
        let mut tree: Tree<i32> = Tree::new(111);
        assert_eq!(*tree.get_value(tree.get_root()).unwrap(), 111);
        let child1 = tree.add_child(0, 12).unwrap();
        let child2 = tree.add_child(0, 32).unwrap();
        let children = tree.get_children(tree.get_root()).unwrap();
        assert_eq!(children.len(), 2);
        assert!(children.contains(&child1));
        assert!(children.contains(&child2));
        assert_eq!(tree.get_parent(child1).unwrap(), tree.get_root());
        assert_eq!(tree.get_parent(child2).unwrap(), tree.get_root());
    }

    #[test]
    fn remove_child() {
        let mut tree: Tree<i32> = Tree::new(111);
        assert_eq!(*tree.get_value(tree.get_root()).unwrap(), 111);
        let child1 = tree.add_child(tree.get_root(), 12).unwrap();
        let child2 = tree.add_child(tree.get_root(), 32).unwrap();
        assert!(tree.remove_entry(child1).is_ok());
        let children = tree.get_children(tree.get_root()).unwrap();
        assert_eq!(children.len(), 1);
        assert!(children.contains(&child2));
    }

    #[test]
    fn mutate_value() {
        struct Thing {
            pub num: i32,
        }
        let mut tree: Tree<Thing> = Tree::new(Thing { num: 111 });
        tree.add_child(0, Thing { num: 12 }).unwrap();
        tree.add_child(0, Thing { num: -9 }).unwrap();
        assert_eq!(tree.get_value(1).unwrap().num, 12);
        assert_eq!(tree.get_value(2).unwrap().num, -9);
        let node = tree.value_mut(1).unwrap();
        node.num /= 4;
        let node = tree.value_mut(2).unwrap();
        node.num = 8;
        assert_eq!(tree.get_value(1).unwrap().num, 3);
        assert_eq!(tree.get_value(2).unwrap().num, 8);
    }
}
