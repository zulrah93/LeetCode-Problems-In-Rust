impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 1;
        }
        
        let mut sorted_nums : Vec<&i32> = nums.iter().filter(|x| *x > &0).collect();
        
        if sorted_nums.is_empty() {
            return 1;
        }
        
        sorted_nums.sort();

        let max = if sorted_nums[sorted_nums.len()-1] == &std::i32::MAX {
            *sorted_nums[sorted_nums.len()-1]
        }
        else {
            sorted_nums[sorted_nums.len()-1] + 1
        };

        for i in 1..max {
            if !sorted_nums.contains(&&i) {
                return i;
            }
        }

        max
        
    }
}