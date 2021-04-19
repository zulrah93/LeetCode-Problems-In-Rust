impl Solution {
    pub fn is_number(s: String) -> bool {
        
        if s.is_empty() {
            return false;
        }
        else if s == "." {
            return false;
        }
        
        let characters : Vec<char> = s.chars().collect();
        
        let last_char = characters.last().unwrap();
        
        if *last_char == 'e' || *last_char == 'E' || *last_char == '-' || *last_char == '+' {
            return false;
        }
        
        let mut matched_e = false;
        let mut matched_point = false;
        let mut digit_found = false;
        
        for (i, c) in characters.iter().enumerate() {
            
            if (*c == 'e' || *c == 'E') && i == 0 { // Can't start with an e/E
                return false;
            }
            else if (*c == 'e' || *c == 'E') && !matched_e { // Mark start of e
                
                matched_e = true;
                
                if i > 0 {
                
                    let previous = &characters[i-1];
                    if !previous.is_digit(10) && !digit_found {
                        return false;
                    }
                    
                }
            }
            else if (*c == 'e' || *c == 'E') && matched_e { // Only a single e/E allowed in scientific notiation
                return false;
            }
            
            if *c == '.' && matched_e {
                return false;
            }
            else if *c == '.' && !matched_point {
                matched_point = true;
            }
            else if *c == '.' && matched_point {
                return false;
            }
            
            if (*c == '+' || *c == '-') && i == 0 {
                continue;
            }
            else if (*c == '+' || *c == '-') && !matched_e {
                return false;
            }
            else if (*c == '+' || *c == '-') {
                
                let previous = &characters[i-1];
                if previous.is_digit(10) {
                    return false;
                }
            }
            
            
            if !c.is_digit(10) && *c != 'e' && *c != 'E' && *c != '.' && *c != '+' && *c != '-' {
                return false;
            }
            else if c.is_digit(10) {
                digit_found = true;
            }
            
            
        }
        
        if *last_char == '.' && !digit_found {
            return false;
        }
        
        return true;
        
    }
}