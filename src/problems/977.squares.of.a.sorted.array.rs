fn square(i32_ref: &i32) -> i32 {
    // Helper method so we don't have to show the dereference in the closure that we pass to the map adapter
    let v = *i32_ref;
    v * v
}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut squares = nums.iter().map(|v_ref| square(v_ref)).collect::<Vec<i32>>(); // We are going to sort so we need to make the variable mutable
        squares.sort();
        squares
    }
}
