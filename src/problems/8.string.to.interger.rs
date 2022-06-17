impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let v : Vec<char> = s.trim_start().chars().collect();
        let mut number_string = String::default();
        let mut leading_number = false;
        let mut is_negative = false;
        let mut has_sign = false;
        for i in 0..v.len() {
            if i == 0 && v[i] == '+' {
                has_sign = true;
            }
            else if i == 0 && v[i] == '-' {
                has_sign = true;
                is_negative = true;
            }
            else if v[i].is_numeric() {
                if i == 0 || (i == 1 && has_sign) {
                    leading_number = true;
                }
                number_string.push(v[i]);
            }
            else if !leading_number {
                return 0;
            }
            else if leading_number {
                break;
            }
        }
        convert(&number_string, is_negative) as i32        
    }
}

fn convert(s : &String, is_negative : bool) -> i128 {
    let mut result : i128 = 0;
    let mut power : u32 = 0;
    let mut base = if is_negative {-1} else {1};
    for c in s.chars().rev() {
        if c == '-' {
            base = -1;
            continue;
        }
        let digit = ((c as u8) - ('0' as u8)) as i128;
        result += (base * (digit * 10i128.pow(power)));
        if result > (i32::MAX as i128) {
            return i32::MAX as i128;
        }
        else if result < (i32::MIN as i128) {
            return i32::MIN as i128;
        }
        power += 1;
    }
    result
}