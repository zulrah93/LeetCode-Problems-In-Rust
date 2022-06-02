impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let mut buffer = nums.clone(); // Space complexity O(N)

        for (index, value) in nums
            .iter()
            .enumerate() // We use the enumerate() higher order function to get the index along with a reference to its value
            .map(|(index, value)| ((index + (k as usize)) % nums.len(), *value))
        // We use the map() higher order to rotate the index and also remove the reference
        {
            buffer[index] = value;
        }

        *nums = buffer;
    }
}
