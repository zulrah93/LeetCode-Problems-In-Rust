impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        
        let mut result : Vec<Vec<i32>> = vec![];
        
        for i in 0..(num_rows as usize) {
            if i == 0 {
                result.push(vec![1]);
            }
            else if i == 1 {
                result.push(vec![1,1]);
            }
            else {
                
                let mut current_row : Vec<i32> = vec![];
                let previous_row = &result[i-1];
                current_row.push(1);
            
                for j in 0..previous_row.len()-1 {
                    current_row.push(previous_row[j] + previous_row[j+1]);
                }
            
                current_row.push(1);
                result.push(current_row);
            }
        }
        
        result
    }
}