//Two  Sum
//Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complements= HashMap::new();
        for (i, n) in nums.into_iter().enumerate() {
            let complement = target - n;
            match complements.get(&complement)  {
                Some(&prev_idx) =>{return vec![prev_idx as i32, i as i32];  
                }
                None=> {complements.insert(n, i as i32);
                }
            }

        }
        unreachable!()
    }
}