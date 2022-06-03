impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        // In order to use binary search we most sort the vector
        let mut sorted_nums = nums.clone(); // Space complexity O(N)
        sorted_nums.sort();
        // Using iterators and higher order functions -- can be done using loops but lines are precious real estate 🛖🏚️🏠🏡🏘️
         (1..((nums.len() as i32)+1)).filter(|x| sorted_nums.binary_search(x).is_err()).collect()
    }
}