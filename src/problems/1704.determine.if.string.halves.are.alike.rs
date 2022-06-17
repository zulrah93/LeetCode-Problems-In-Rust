fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let length = s.len();
        let vector = s
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<char>>();
        let first_half_vowel_count = vector[0..(length / 2)]
            .iter()
            .filter(|c| is_vowel(c))
            .count();
        let second_half_vowel_count = vector[(length / 2)..]
            .iter()
            .filter(|c| is_vowel(c))
            .count();
        first_half_vowel_count == second_half_vowel_count
    }
}

#[test]
fn simple_test_case() {
    assert_eq!(Solution::halves_are_alike("book".to_string()), true);
    assert_eq!(Solution::halves_are_alike("textbook".to_string()), false);
    assert_eq!(Solution::halves_are_alike("Uo".to_string()), true);
}
