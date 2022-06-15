/*
    WORK IN PROGRESS [⚠️ Incomplete Solution ⚠️]
*/

fn is_even_length(string: &String) -> bool {
    string.len() % 2 == 0
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let length = s.len(); // Used to determine mid point of string
        let chars = s.chars().collect::<Vec<char>>();

        let mut start_index = 0usize;
        let mut end_index = 1usize; // We default to 1 so we can at least return a single character which by definition is the smallest palidrone

        if is_even_length(&s) {
            // Case: Even length string which is like odd case

            let midpoint_index_a = (length / 2) - 1; // We are going to start at the midpoint
            let midpoint_index_b = midpoint_index_a + 1; // Even length strings has a secondary midpoint

            for offset in 0..(length / 2) {
                let current_left_char = &chars[midpoint_index_a - offset]; // Looking at the left most character from midpoint character
                let current_right_char = &chars[midpoint_index_b + offset]; // Looking at the right most character from midpoint character

                if current_left_char != current_right_char {
                    // Note: Rust never compares references like Java for example so this code is safe
                    break; // We no longer have a palindrome and we essentially no longer update start and end index which will be the bounds of the longest palidromic substring
                }

                start_index = midpoint_index_a - offset;
                end_index = (midpoint_index_b + offset) + 1;
            }
        } else {
            // Case: Odd length string

            let midpoint_index = length / 2; // We are going to start at the midpoint

            for offset in 0..((length / 2) + 1) {
                // O(N/2) where N is |s|

                let current_left_char = &chars[midpoint_index - offset]; // Looking at the left most character from midpoint character
                let current_right_char = &chars[midpoint_index + offset]; // Looking at the right most character from midpoint character

                if current_left_char != current_right_char {
                    // Note: Rust never compares references like Java for example so this code is safe
                    break; // We no longer have a palindrome and we essentially no longer update start and end index which will be the bounds of the longest palidromic substring
                }

                start_index = midpoint_index - offset;
                end_index = (midpoint_index + offset) + 1;
            }
        }

        chars[start_index..end_index].iter().collect::<String>()
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
