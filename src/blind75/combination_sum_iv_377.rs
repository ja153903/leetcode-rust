#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        // if we have state, then what we want to set up
        // is an array that is based on the target
        // i.e. dp[i] is how many ways can we get to the sum i
        let mut dp: Vec<i32> = vec![0; (target + 1) as usize];

        dp[0] = 1;

        for i in 1..=target {
            for &num in nums.iter() {
                if i - num >= 0 {
                    dp[i as usize] += dp[(i - num) as usize];
                }
            }
        }

        dp[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let nums: Vec<i32> = vec![1, 2, 3];
        let target: i32 = 4;

        let expected: i32 = 7;
        let result = Solution::combination_sum4(nums, target);

        assert_eq!(result, expected);
    }
}
