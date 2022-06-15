// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root_option: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if is_leaf(&root_option) {
            // Special case if root is leaf we return
            0
        } else {
            let mut result = 0;
            helper(&root_option, &mut result);
            result
        }
    }
}

//A leaf is a node with no children (left and right nodes)
fn is_leaf(node_option: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(node_ref) = node_option {
        let node = node_ref.borrow();
        node.left.is_none() && node.right.is_none()
    } else {
        false
    }
}

fn get_node_value(node_option: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node_ref) = node_option {
        let node = node_ref.borrow();
        node.val
    } else {
        0
    }
}

//Pass the root node and a mutable reference to a 32 bit signed integer
fn helper(node_option: &Option<Rc<RefCell<TreeNode>>>, result: &mut i32) {
    if let Some(node_ref) = node_option {
        let node = node_ref.borrow();
        if is_leaf(&node.left) {
            *result += get_node_value(&node.left);
            helper(&node.right, result);
        } else {
            helper(&node.left, result);
            helper(&node.right, result);
        }
    }
}
