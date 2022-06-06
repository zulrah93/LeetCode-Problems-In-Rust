impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result = String::default();
        for token in s
            .split(' ') // Split string based off whitespace character
            .map(|x| x.to_string().chars().rev().collect::<String>())
        // Map each split string with the reverse
        {
            result.push_str(token.as_str());
            result.push_str(" "); // We will trim the string at the end
        }
        result.trim().to_string()
    }
}
