// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let sum = vector_to_number(&linked_list_to_vector(&l1))
            + vector_to_number(&linked_list_to_vector(&l2));
        vector_to_linked_list(&number_to_vector(sum))
    }
}

fn linked_list_to_vector(linked_list: &Option<Box<ListNode>>) -> Vec<i32> {
    if linked_list.is_none() {
        vec![] // Empty vector will translate to a None option
    } else {
        if let Some(head_node) = linked_list.as_ref() {
            // We want the head by reference so we don't consume the node object by accident
            let mut result = vec![head_node.val];

            loop {
                if let Some(next_node) = head_node.next.as_ref() {
                    result.push(next_node.val);
                } else {
                    break;
                }
            }

            println!("[linked_list_to_vector] result = {:?}", result);

            result
        } else {
            vec![]
        }
    }
}

fn vector_to_number(vector: &Vec<i32>) -> i32 {
    let mut result = 0;

    for (index, value) in vector.iter().enumerate() {
        let x = (vector.len() - (index + 1)) as u32;
        result += (10i32.pow(x) * vector[index])
    }

    println!("[vector_to_number] result = {}", result);

    result
}

fn number_to_vector(number: i32) -> Vec<i32> {
    let mut x = number;
    let mut result = vec![];

    while x > 0 {
        result.insert(0, x % 10); // X MOD BASE -- insert at 0 so we don't
        x /= 10;
    }

    println!("[number_to_vector] result = {:?}", result);

    result
}

fn vector_to_linked_list(vector: &Vec<i32>) -> Option<Box<ListNode>> {
    // ⚠️ WIP ⚠️ -- still won't compile -- building a linked list is harder in Rust than in C++ -- mainly because of Rust's strong safety features
    if vector.is_empty() {
        None
    } else {
        let mut head: &Option<Box<ListNode>> = &None;

        for value in vector {
            let node = Box::new(ListNode::new(*value));

            if head.is_none() {
                // Linked list is empty
                head.replace(node);
            } else if let Some(current_head) = head.as_mut() {
                // Linked list isn't empty so we get mutable reference of the current node
                current_head.next.replace(node);
                head.replace(node);
            }
        }

        Some(head.into())
    }
}
