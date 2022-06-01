impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut non_zero_index = 0;

        for i in 0..nums.len() {
            let x = nums[i];

            if x != 0 {
                nums[non_zero_index] = x;
                non_zero_index += 1;
            }
        }

        for i in non_zero_index..nums.len() {
            nums[i] = 0;
        }
    }
}
