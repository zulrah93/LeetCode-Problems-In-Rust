use std::cmp::max;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        
        let mut index1 = m-1;
        let mut index2 = n-1;
        
        for i in (0..(n+m)).rev() {
            let x = if index1 >= 0 {
                Some(*nums1.get(index1 as usize).unwrap())
            }
            else {
                None
            };
            
            let y = if index2 >= 0 {
                Some(*nums2.get(index2 as usize).unwrap())
            }
            else {
                None
            };
            
            match (x,y) {
                (Some(x), None) => {
                    
                    nums1[i as usize] = x;                    
                    index1 -= 1;
                },
                (None, Some(y)) => {
                   
                    nums1[i as usize] = y;                    
                    index2 -= 1;
                },
                (Some(x), Some(y)) => {
                    
                    let m = max(x,y);
                    
                    nums1[i as usize] = m;
                   
                    if m == x {
                        index1 -= 1;
                    }
                    else {
                        index2 -= 1;
                    }                    
                },
                (None,None) => {
                    continue;
                }
            }
        }
         
    }
}