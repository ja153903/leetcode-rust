#![allow(dead_code)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            if seen.contains_key(&(target - num)) {
                let j = seen.get(&(target - num)).unwrap();

                return vec![*j as i32, i as i32];
            }

            seen.insert(num, i);
        }

        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_test_case1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let expected = vec![0, 1];

        assert_eq!(Solution::two_sum(nums, target), expected);
    }
}
