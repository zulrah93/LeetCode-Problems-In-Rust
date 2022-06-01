impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .enumerate()
            .map(|(index, value_ref)| {
                let mut sum = *value_ref;
                for i in 0..index {
                    sum += nums[i];
                }
                sum
            })
            .collect()
    }
}
