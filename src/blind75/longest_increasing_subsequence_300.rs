#![allow(dead_code)]

struct Solution;

use std::cmp;

impl Solution {
    // what is the gist of the solution here?
    // if this was a dynamic programming exercise, how would we set up our state?
    // The idea here is that given some index j, we'd want to know what previous point i
    // would it have been a good idea to increase the length of the sequence i.e. which gives us
    // the maximum length up to j from any previous i.
    // this would mean that this algorithm should run in O(n^2)
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![1; nums.len() + 1];

        // if we let the dp here mean that up to index i, this is the max len
        for i in 1..nums.len() {
            for j in 0..=i {
                if nums[j] < nums[i] {
                    dp[i] = cmp::max(dp[i], dp[j] + 1);
                }
            }
        }

        if let Some(&value) = dp.iter().max() {
            value
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let nums: Vec<i32> = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let expected: i32 = 4;

        let result: i32 = Solution::length_of_lis(nums);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case2() {
        let nums: Vec<i32> = vec![0, 1, 0, 3, 2, 3];
        let expected: i32 = 4;

        let result: i32 = Solution::length_of_lis(nums);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case3() {
        let nums: Vec<i32> = vec![7, 7, 7, 7, 7, 7, 7];
        let expected: i32 = 1;

        let result: i32 = Solution::length_of_lis(nums);

        assert_eq!(result, expected);
    }
}
