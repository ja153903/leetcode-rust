#![allow(dead_code)]

struct Solution;

use std::cmp;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        if nums.len() == 1 {
            return nums[0];
        }

        // let dp[i] be the maximum amount you can rob up to house i
        // to determine this, we can say that the maximum value you can rob at house i
        // should be the previous state dp[i-1] or dp[i-2] + nums[i]
        let mut dp: Vec<i32> = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = cmp::max(nums[0], nums[1]);

        for i in 2..nums.len() {
            dp[i] = cmp::max(dp[i - 1], dp[i - 2] + nums[i]);
        }

        dp[nums.len() - 1]
    }

    pub fn rob_optimal(nums: Vec<i32>) -> i32 {
        let mut pre: i32 = 0;
        let mut cur: i32 = 0;

        for &num in nums.iter() {
            let temp = cmp::max(pre + num, cur);
            pre = cur;
            cur = temp;
        }

        cur
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let nums: Vec<i32> = vec![1, 2, 3, 1];
        let expected: i32 = 4;

        let result: i32 = Solution::rob(nums);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case2() {
        let nums: Vec<i32> = vec![1, 2, 3, 1];
        let expected: i32 = 4;

        let result: i32 = Solution::rob_optimal(nums);

        assert_eq!(result, expected);
    }
}
