static RADIX: i32 = 10;

fn is_out_of_i32_range(result: i64) -> bool {
    if result < (i32::MIN as i64) {
        // Result is below the mininum of a 32-bit signed integer
        return true;
    }
    if result > (i32::MAX as i64) {
        // Result is above the maximum of a 32-bit unsigned integer
        return true;
    }
    return false; // It's in range and safe to down cast i64 to i32
}

fn vector_to_i64(buffer: &Vec<i32>, is_negative: bool) -> i64 {
    let mut result = 0;

    for (index, value) in buffer.iter().rev().enumerate() {
        let value = *value as i64;
        result += (RADIX as i64).pow(index as u32) * value;
    }

    result * (if is_negative { -1 } else { 1 })
}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_negative = x < 0;
        let mut x = if is_negative { x * -1 } else { x };
        let mut buffer = vec![]; // Note: if x = 31 then buffer will be [1, 3]

        while x > 0 {
            buffer.push(x % RADIX);
            x /= RADIX;
        }

        let result = vector_to_i64(&buffer, is_negative);

        if is_out_of_i32_range(result) {
            0
        } else {
            result as i32
        }
    }
}

#[test]
fn negative_test_case() {
    assert_eq!(Solution::reverse(-123), -321);
}