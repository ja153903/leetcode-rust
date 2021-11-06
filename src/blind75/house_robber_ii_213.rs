#![allow(dead_code)]

struct Solution;

use std::cmp;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // this is similar to the first house robber problem, the only diff here is that we need to
        // run two versions of the problem. One from 0 to n - 2 and the other from 1 to n - 1
        if nums.is_empty() {
            0
        } else if nums.len() == 1 {
            nums[0]
        } else {
            cmp::max(
                Solution::rob_helper(&nums, 0, nums.len() - 2),
                Solution::rob_helper(&nums, 1, nums.len() - 1),
            )
        }
    }

    // algorithm from the first solution
    pub fn rob_helper(nums: &[i32], i: usize, j: usize) -> i32 {
        let mut pre: i32 = 0;
        let mut cur: i32 = 0;

        for &num in nums.iter().take(j + 1).skip(i) {
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
        let nums: Vec<i32> = vec![2, 3, 2];
        let expected: i32 = 3;

        let result: i32 = Solution::rob(nums);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case2() {
        let nums: Vec<i32> = vec![1, 2, 3, 1];
        let expected: i32 = 4;

        let result: i32 = Solution::rob(nums);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case3() {
        let nums: Vec<i32> = vec![1, 2, 3];
        let expected: i32 = 3;

        let result: i32 = Solution::rob(nums);

        assert_eq!(result, expected);
    }
}
