impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let nums = (1..(n + 1)).collect::<Vec<i32>>();
        let power_set = (2usize).pow(nums.len() as u32);
        for i in 1..power_set {
            if i.count_ones() != (k as u32) {
                continue;
            }
            let mut current = vec![];
            let mut j = i;
            let mut index = 0;
            while j > 0 {
                if j & 1 == 1 {
                    current.push(nums[index]);
                }
                j >>= 1;
                index += 1;
            }
            current.sort();
            if !result.contains(&current) {
                result.push(current);
            }
        }
        result
    }
}
