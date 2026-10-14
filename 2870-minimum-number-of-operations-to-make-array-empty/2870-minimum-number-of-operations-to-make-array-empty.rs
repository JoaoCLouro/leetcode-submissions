use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut h = HashMap::<i32, i32>::new();
        for &n in &nums {
            *h.entry(n).or_insert(0) += 1;
        }
    
        let mut count = 0;
        for &val in h.values() {
            if  val == 1{
                return -1;
            }
            else {
                count += (val + 2) / 3
            }
        }
        count
    }
}