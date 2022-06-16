use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for num in nums {
            heap.push(num);
        }

        let mut answer = 0;
        for _ in 0..k {
            answer = heap.pop().unwrap_or_default();
        }

        answer
    }
}
