fn letter_to_digit(letter: &char) -> i32 {
    // Note: Only works if char is ASCII
    let byte = *letter as u8;
    let offset = 'A' as u8;
    ((byte - offset) + 1) as i32
}

#[test]
fn test_letter_to_digit() {
    assert_eq!(letter_to_digit(&'A'), 1);
    assert_eq!(letter_to_digit(&'B'), 2);
    assert_eq!(letter_to_digit(&'C'), 3);
    assert_eq!(letter_to_digit(&'D'), 4);
    assert_eq!(letter_to_digit(&'E'), 5);
    assert_eq!(letter_to_digit(&'F'), 6);
    assert_eq!(letter_to_digit(&'G'), 7);
    assert_eq!(letter_to_digit(&'H'), 8);
    assert_eq!(letter_to_digit(&'I'), 9);
    assert_eq!(letter_to_digit(&'J'), 10);
    assert_eq!(letter_to_digit(&'K'), 11);
    assert_eq!(letter_to_digit(&'L'), 12);
    assert_eq!(letter_to_digit(&'M'), 13);
    assert_eq!(letter_to_digit(&'N'), 14);
    assert_eq!(letter_to_digit(&'O'), 15);
    assert_eq!(letter_to_digit(&'P'), 16);
    assert_eq!(letter_to_digit(&'Q'), 17);
    assert_eq!(letter_to_digit(&'R'), 18);
    assert_eq!(letter_to_digit(&'S'), 19);
    assert_eq!(letter_to_digit(&'T'), 20);
    assert_eq!(letter_to_digit(&'U'), 21);
    assert_eq!(letter_to_digit(&'V'), 22);
    assert_eq!(letter_to_digit(&'W'), 23);
    assert_eq!(letter_to_digit(&'X'), 24);
    assert_eq!(letter_to_digit(&'Y'), 25);
    assert_eq!(letter_to_digit(&'Z'), 26);
}

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let length = column_title.len();
        let tokens = column_title.chars().collect::<Vec<char>>();
        let radix = 26i32;
        let mut result = 0;

        for (power, letter) in tokens.iter().enumerate().map(|t| (length - t.0 - 1, t.1)) {
            result += letter_to_digit(letter) * radix.pow(power as u32);
        }

        result
    }
}

#[test]
fn test_title_to_number() {
    assert_eq!(Solution::title_to_number("AA".to_string()), 27);
}
