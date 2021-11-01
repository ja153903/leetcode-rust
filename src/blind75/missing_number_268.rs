#![allow(dead_code)]

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut xor = 0;
        let mut i = 0;

        while i < nums.len() {
            // note that this works because xor is transitive
            // this means that at some point we'll cancel out nums[index] = index
            xor = xor ^ i ^ nums[i] as usize;
            i += 1;
        }

        (xor ^ i) as i32
    }

    // This solution is a straightforward one
    // however it is quite slow compared to the ideal solution
    pub fn hash_set_solution(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();

        for &num in nums.iter() {
            seen.insert(num);
        }

        for i in 0..=nums.len() {
            if !seen.contains(&(i as i32)) {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let nums: Vec<i32> = vec![3, 0, 1];
        let expected: i32 = 2;

        let result = Solution::missing_number(nums);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case2() {
        let nums: Vec<i32> = vec![3, 0, 1];
        let expected: i32 = 2;

        let result = Solution::hash_set_solution(nums);

        assert_eq!(result, expected);
    }
}
