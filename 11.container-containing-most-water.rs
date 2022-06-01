use std::cmp::{max, min};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0usize;
        let mut j = height.len() - 1;
        let mut max_area = 0;

        while (i < j) {
            let x = &height[i];
            let y = &height[j];
            let min_height = min(*x, *y);

            max_area = max(max_area, min_height * (j - i) as i32);

            if *x < *y {
                i += 1;
            } else {
                j -= 1;
            }
        }

        max_area
    }
}
