use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        let mut x = n;

        while x != 1 {
            let mut y = 0;

            while x > 0 {
                y += (x % 10).pow(2) as i32;
                x /= 10;
            }

            if set.contains(&y) {
                return false;
            } else {
                set.insert(y);
            }

            x = y;
        }

        true
    }
}
