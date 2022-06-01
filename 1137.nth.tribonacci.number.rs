struct Tribonacci {
    current_a: i32,
    current_b: i32,
    next: i32,
}

impl Default for Tribonacci {
    fn default() -> Self {
        Self {
            current_a: 0,
            current_b: 1,
            next: 1,
        }
    }
}

impl Iterator for Tribonacci {
    type Item = i32; // We are required create an alias of i32 because the trait next returns Self::Item and Item is a user defined type

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current_a + self.current_b + self.next;
        //println!("{} {} {} = {}", self.current_a, self.current_b, self.next, new_next);
        self.current_a = self.current_b;
        self.current_b = self.next;
        self.next = new_next;
        Some(new_next)
    }
}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        // Recursive implementation:

        // let n = n - 1; // Make it zero-based
        // match n {
        //     0 => 0,
        //     1 => 1,
        //     2 => 1,
        //     _ => Solution::tribonacci(n) + Solution::tribonacci(n-1) + Solution::tribonacci(n-2)
        // }

        if n < 3 {
            return match n {
                0 => 0,
                1 => 1,
                2 => 1,
                _ => -1, // Should never hit here because of the if
            };
        }

        // Iterator implementation (Lazy evaluation):
        Tribonacci::default()
            .nth((n - 3) as usize) // First item in iterator is actually the 4th tribonacci number
            .unwrap_or_default()
    }
}
