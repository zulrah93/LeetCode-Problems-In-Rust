/*
    WORK IN PROGRESS [⚠️ Incomplete Solution ⚠️]
*/

fn pattern_has_no_wildcard(pattern: &String) -> bool {
    pattern
        .chars()
        .filter(|c| get_wildcard_type(c) == WildCard::None) // Only keep the none special characters
        .collect::<String>()
        .eq(pattern) // Check if it matches with the original pattern and if true the pattern has no special characters
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        println!("Input: {} Pattern: {}", s, p);
        let no_wildcards = pattern_has_no_wildcard(&p);
        if no_wildcards {
            // No wildcards
            s == p
        } else if p == "*".to_string() {
            // Case where the pattern is just a kleene star (Accept any input string)
            true
        } else {
            is_match_dfs(&s.chars().collect(), 0, &p.chars().collect(), 0) // We call helper method and write our logic to see if the input is a match
        }
    }
}

#[derive(PartialEq, Debug)]
enum WildCard {
    Single,
    Any,
    None,
}

fn get_wildcard_type(character: &char) -> WildCard {
    match character {
        '?' => WildCard::Single,
        '*' => WildCard::Any,
        _ => WildCard::None,
    }
}

//This function will make recursive calls so we are approaching this problem using Depth First Search [https://en.wikipedia.org/wiki/Depth-first_search]
fn is_match_dfs(
    input: &Vec<char>,
    input_index: usize,
    pattern: &Vec<char>,
    pattern_index: usize,
) -> bool {
    if input_index < input.len() {
        if pattern_index >= pattern.len() {
            // We have an input greater than the pattern given for example [ input: "a"  pattern: "aa" ]
            return false;
        }

        let current_pattern = &pattern[pattern_index];
        let current_input = &input[input_index];

        let input_pattern_match = current_input == current_pattern;
        let wildcard_type = get_wildcard_type(current_pattern); //
        let pattern_at_end = (pattern_index + 1) == pattern.len(); // Kleene star is at the end [special case]
        let pattern_at_start = pattern_index == 0; // Kleene star is at the start  [special case]

        println!(
            "input[{}] = {} pattern[{}] = {}",
            input_index, current_input, pattern_index, current_pattern
        );

        match wildcard_type {
            WildCard::None => {
                if input_pattern_match {
                    is_match_dfs(input, input_index + 1, pattern, pattern_index + 1)
                } else {
                    false
                }
            }
            WildCard::Single => {
                return is_match_dfs(input, input_index + 1, pattern, pattern_index + 1);
            }
            WildCard::Any => {
                if pattern_at_end {
                    // If the ending pattern is a kleene star our job is done because we can match anything including an empty string
                    true
                } else if pattern_at_start {
                    // Note: we already checked for '*' before the helper method was called

                    let next_pattern = &pattern[pattern_index + 1];
                    let mut index = input_index;
                    let mut passed = false; // If set to true

                    while index < input.len() {
                        let current = &input[index];
                        if current == next_pattern {
                            passed = true;
                            break;
                        }
                        index += 1;
                    }

                    if passed {
                        // If we patch we move the input index by one to advance to the next set of symbols
                        is_match_dfs(input, index + 1, pattern, pattern_index + 2)
                    } else {
                        false
                    }
                } else {
                    let next_pattern = &pattern[pattern_index + 1];
                    let input_pattern_match = next_pattern == current_input; // Note: This overrides the local variable outside of the match scope; so we keep consistency in the variable names

                    return if input_pattern_match {
                        is_match_dfs(input, input_index + 1, pattern, pattern_index + 1)
                    } else {
                        is_match_dfs(input, input_index + 1, pattern, pattern_index)
                        // We keep the position at the kleene star
                    };
                }
            }
        }
    } else {
        true
    }
}

// Test cases taken from LeetCode

#[test]
fn case_1() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
}

#[test]
fn case_2() {
    assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::is_match("cb".to_string(), "?a".to_string()),
        false
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::is_match("adceb".to_string(), "*a*b".to_string()),
        true
    );
}

#[test]
fn case_5() {
    assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
}

#[test]
fn case_6() {
    assert_eq!(
        Solution::is_match("acdcb".to_string(), "a*c?b".to_string()),
        false
    );
}

#[test]
fn case_7() {
    assert_eq!(Solution::is_match("ab".to_string(), "?*".to_string()), true);
}

#[test]
fn case_8() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
}

#[test]
fn case_9() {
    assert_eq!(Solution::is_match("a".to_string(), "aa".to_string()), false);
}

#[test]
fn case_10() {
    assert_eq!(
        Solution::is_match("abcabczzzde".to_string(), "*abc???de*".to_string()),
        false
    );
}

// Custom input case

#[test]
fn custom_case_1() {
    assert_eq!(
        Solution::is_match("ddradceb".to_string(), "*a*b".to_string()),
        true
    );
}
