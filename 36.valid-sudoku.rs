use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        
        for r in 0..9 {
            let row = &board[r];
            let mut set : HashSet<char> = HashSet::new();
            for c in 0..9 {
                let value = &row[c];
                if *value == '.' {
                    continue;
                }
                if set.contains(value) {
                    return false;
                }
                set.insert(row[c]);
            }
        }
        
        for c in 0..9 {
            let mut set : HashSet<char> = HashSet::new();
            for r in 0..9 {
                let row = &board[r];
                let value = &row[c];
                if *value == '.' {
                    continue;
                }
                if set.contains(value) {
                    return false;
                }
                set.insert(row[c]);
            }
        }
        

        let mut c = 0;
        
        while c < 9 {
            
            let mut r = 0;
        
            while r < 9 {
                let mut set = HashSet::new();
                for i in r..(r+3) {
                    let row = &board[i];
                    for j in c..(c+3) {
                        let value = &row[j];
                        if *value == '.' {
                            continue;
                        }
                        if set.contains(value) {
                            return false;
                        }
                        set.insert(row[j]);
                    }
                }
                r += 3;
            }

            c += 3;
            
        }
        
        true
    }
}