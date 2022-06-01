use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums_sorted = nums.to_vec();
        nums_sorted.sort();

        let size = nums_sorted.len();

        let mut result = HashSet::new();

        for i in 0..size {
            // O(N^2*Log(N))
            let x = &nums_sorted[i];
            for j in 0..size {
                if i != j {
                    let y = &nums_sorted[j];
                    let target = -(*x + *y);
                    if let Ok(target_index) = nums_sorted.binary_search(&target) {
                        if target_index != j && target_index != i {
                            let mut triplet = vec![*x, *y, target];
                            triplet.sort();
                            result.insert(triplet);
                        }
                    }
                }
            }
        }

        result.into_iter().collect()
    }
}
