impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut result: Vec<i32> = vec![];

        result.append(&mut nums1.clone());
        result.append(&mut nums2.clone());
        result.sort();

        let middle_index = result.len() / 2;

        if result.len() % 2 == 0 {
            return ((result[middle_index] + result[middle_index - 1]) as f64 / 2 as f64);
        } else {
            result[middle_index] as f64
        }
    }
}
