impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() || !nums.contains(&target) {
            return vec![-1, -1];
        }

        let nums_with_index: Vec<_> = nums.iter().enumerate().filter(|x| *x.1 == target).collect();

        let first = nums_with_index.first().unwrap();

        let last = nums_with_index.last().unwrap();

        vec![first.0 as i32, last.0 as i32]
    }
}
