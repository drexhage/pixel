use std::ops::Div;

use crate::Tree;

/// `tr` creates a tree consisting only of the root node in which the given value is stored.
///
/// Together with the divide operator, we get a good shorthand notation for defining trees.
/// The divide operator `a / b` adds the whole tree of `b` as a child to the tree of `a`.
/// Similarly, `a / b / c` adds the whole tree of `b` and `c` as children to the tree `a`.
///
/// # Example
///
/// The tree:
/// ```text
///        0
///     /     \
///    1       2
///   / \     / \
///  3   4   5   6
/// ```
///
/// Is being created by this expression:
///
/// ```
/// use baum::tr;
/// let tree = tr(0) / (tr(1) / tr(2) / tr(3)) / (tr(4) / tr(5) / tr(6));
/// ```
pub fn tr<T>(value: T) -> Tree<T> {
    Tree::new(value)
}

impl<T> Div for Tree<T> {
    type Output = Tree<T>;

    fn div(mut self, rhs: Self) -> Self::Output {
        self.add_tree_as_child(rhs, self.get_root()).unwrap();
        self// moving the tree value into this function and so it can be returned again
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_children_values(tree: &Tree<i32>, idx: usize) -> Vec<i32> {
        tree.get_children(idx)
            .unwrap()
            .into_iter()
            .map(|x| *tree.get_value(x).unwrap())
            .collect()
    }

    #[test]
    fn test() {
        //       0
        //    /     \
        //   6       2
        //  / \     / \
        // 1   4   3   5
        let tree = tr(0) / (tr(6) / tr(1) / tr(4)) / (tr(2) / tr(3) / tr(5));
        let children0 = get_children_values(&tree, tree.get_root());
        assert!(children0.contains(&6) && children0.contains(&2) && children0.len() == 2);
    }

    #[test]
    fn test_equality() {
        let t1 = tr(1) / tr(2) / tr(3);
        let t2 = tr(1) / tr(2) / tr(3);
        let t3 = tr(1) / tr(3) / tr(2);
        let t4 = tr(1) / tr(2) / tr(4);
        let t5 = tr(4) / tr(2) / tr(3);
        assert!(t1 == t2);
        assert!(t1 != t3 && t2 != t3);
        assert!(t1 != t4 && t2 != t4);
        assert!(t1 != t5 && t2 != t5);
    }
}
