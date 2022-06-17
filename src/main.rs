struct Solution {}

fn evaluate(operation : &str, result : &mut i32) {
    match operation {
        "++X" | "X++" => *result += 1,
        "--X" | "X--" => *result -= 1,
        _ => ()
    }
}

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut result = 0;
        for operation in operations.iter().map(|o| o.as_str()) {
            evaluate(operation, &mut result);
        }
        result
    }
}

fn main() {
    println!("Scratch Space Used To Write Unit Tests For LeetCode Problem And Enable Lint Space. This Project Isn't Meant To Be Compiled And Run.");
}
