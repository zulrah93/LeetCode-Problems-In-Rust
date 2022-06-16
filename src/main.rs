use std::cmp::max;

fn get_tuple_length(tuple : &(usize, usize)) -> usize {
    (tuple.1 - tuple.0) + 1
}

fn expand_around_center(string_vec : &Vec<char>, left_index : usize, right_index : usize) -> (usize,usize) {

    let length = string_vec.len();
    let mut left_index = left_index as i64;
    let mut right_index = right_index;

    while left_index >= 0 && right_index < length { // Worst case complexity: O(N) with constant space

        let left_char = &string_vec[left_index as usize];
        let right_char = &string_vec[right_index];
        if left_char != right_char { // Note: Rust doesn't compare by reference so this is considered safe unlike Java for example
            break; // We break out of loop since we no longer have a palindrome
        }
        left_index -= 1;
        right_index += 1;

    }

    let left_index = (left_index + 1) as usize;
    let right_index = right_index;

    let result = (left_index,right_index); // The tuple returned contains the starting index and ending index (+1) of the largest palindrome

    result 

}

struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {

        //println!("Finding Longest Palindrome For: [{}]", s);

        if s.is_empty() {
            return String::default();
        }

        let length = s.len();
        let string_vec = s.chars().collect::<Vec<char>>();
        let mut start_index = 0usize;
        let mut end_index = 1usize;

        for index in 0..length {

            let odd_tuple = expand_around_center(&string_vec, index, index);
            let even_tuple = expand_around_center(&string_vec, index, index+1); // Even length means they are two centers or visually an empty character in between the two center characters
            
            let max_tuple = if get_tuple_length(&odd_tuple) >= get_tuple_length(&even_tuple) {
                odd_tuple
            }
            else {
                even_tuple
            };

            if get_tuple_length(&max_tuple) >= get_tuple_length(&(start_index, end_index)) {
                start_index = max_tuple.0;
                end_index = max_tuple.1; 
            }
            
        }

        string_vec[start_index..end_index].iter().collect()

    }
}

#[test]
fn odd_length_test_case_1() {
    assert_eq!(
        "aba".to_string(),
        Solution::longest_palindrome("babad".to_string())
    );
}

#[test]
fn odd_length_test_case_2() {
    assert_eq!(
        "bb".to_string(),
        Solution::longest_palindrome("abb".to_string())
    );
}


#[test]
fn even_length_test_case_1() {
    assert_eq!(
        "bb".to_string(),
        Solution::longest_palindrome("cbbd".to_string())
    );
}

#[test]
fn even_length_test_case_2() {
    assert_eq!(
        "a".to_string(),
        Solution::longest_palindrome("ac".to_string())
    );
}


fn main() {
    println!("Scratch Space Used To Write Unit Tests For LeetCode Problem And Enable Lint Space. This Project Isn't Meant To Be Compiled And Run.");
}
