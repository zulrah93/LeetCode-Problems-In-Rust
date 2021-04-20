impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        
        let s_lower : Vec<char> = s.to_lowercase().chars().filter(|x| x.is_ascii_alphanumeric()).collect();
        
        for (index, c) in s_lower.iter().enumerate() {
            
            let c2 = &s_lower[s_lower.len() - index - 1];
            
            if *c != *c2 {
                return false;
            }
            
        }
        
        return true;
        
    }
}