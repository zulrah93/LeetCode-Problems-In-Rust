// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut array1 : Vec<i32> = vec![];
        let mut array2 : Vec<i32> = vec![];
        if let Some(root_ref) = root {
            
            let root_node = root_ref.borrow();
            tree_to_array(&root_node.left, &mut array1, false);
            tree_to_array(&root_node.right, &mut array2, true);
            
            array1 == array2
        }
        else {
            return true;
        }
    }
}


fn tree_to_array(root: &Option<Rc<RefCell<TreeNode>>>, result : &mut Vec<i32>, mirror : bool) {
    
    if let Some(node_ref) = root {
        
        let node = node_ref.borrow();
        
        
        result.push(node.val);
        
        if mirror {
            tree_to_array(&node.left, result, mirror);
            tree_to_array(&node.right, result, mirror);
        }
        else {
            tree_to_array(&node.right, result, mirror);
            tree_to_array(&node.left, result, mirror);
        }
    }
    else {
        result.push(-101);
    }
} 
