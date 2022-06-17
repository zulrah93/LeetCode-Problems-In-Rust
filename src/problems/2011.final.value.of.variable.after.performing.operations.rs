fn evaluate(operation: &str, result: &mut i32) {
    match operation {
        // Match makes this more elegant in my opinion 😍
        "++X" | "X++" => *result += 1,
        "--X" | "X--" => *result -= 1,
        _ => (),
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
