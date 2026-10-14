use std::collections::BinaryHeap;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, s: String) -> i64 {
        let mut sc: Vec<char> = s.chars().collect();
        let mut h = BinaryHeap::new();
        let mut score: i64 = 0;

        for i in 0..nums.len() {
           h.push(nums[i]);

           if sc[i] == '1' {
            score += h.pop().unwrap() as i64;
           }
        }
        score
    }
}