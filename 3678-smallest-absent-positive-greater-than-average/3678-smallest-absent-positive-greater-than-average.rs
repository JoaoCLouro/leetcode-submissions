use std::collections::HashMap;

impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut h = HashMap::<i32, bool>::new();

        for &n in &nums {
            sum += n;
            *h.entry(n).or_insert(true);
        }
        let med = sum / nums.len() as i32;

        let mut result = if med < 0 {1} else {med + 1};
        loop {
            if h.contains_key(&result) {
                result += 1;
            }
            else {
                break;
            }
        }
        result
    }
}