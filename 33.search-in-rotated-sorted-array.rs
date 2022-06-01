impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        for (i, v) in nums.iter().enumerate() {
            if *v == target {
                return i as i32;
            }
        }

        -1
    }
}
