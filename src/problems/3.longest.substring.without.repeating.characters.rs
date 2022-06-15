use std::cmp::max;
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let characters: Vec<char> = s.chars().collect();
        let mut max_substr = 0;
        let mut i = 0;
        let mut previous = 0;
        let mut set = HashSet::new();

        loop {
            if i >= characters.len() {
                break;
            }
            let c = &characters[i];
            if !set.insert(*c) {
                max_substr = max(max_substr, set.len());
                set.clear();
                i = previous + 1;
                previous = i;
                continue;
            }
            i += 1;
        }

        max_substr = max(max_substr, set.len());
        max_substr as i32
    }
}
