impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            map.entry(nums[i]).or_insert(vec![]).push(i);
        }
        for i in 0..nums.len() {
            let x = nums[i];
            let y = target - x;
            if map.contains_key(&y) {
                let indices = &map[&y];
                for index in indices {
                    if *index != i {
                        return vec![i as i32, *index as i32];
                    }
                }
            }
        }
        vec![]
    }
}
