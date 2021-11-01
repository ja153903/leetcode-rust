#![allow(dead_code)]

struct Solution;

use std::cmp;
use std::i32;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // dp[i] is the number of coins needed to achieve some amount i
        // in this case, we would then say that dp[i] = min(dp[i - coin] for coin in coins) + 1
        let mut dp: Vec<i32> = vec![i32::MAX; (amount + 1) as usize];
        dp[0] = 0;

        for i in 1..=amount {
            let mut current_min = i32::MAX;
            for &coin in coins.iter() {
                if i - coin >= 0 {
                    current_min = cmp::min(current_min, dp[(i - coin) as usize]);
                }
            }

            if current_min != i32::MAX {
                dp[i as usize] = current_min + 1;
            }
        }

        if dp[amount as usize] == i32::MAX {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let coins: Vec<i32> = vec![1, 2, 5];
        let amount: i32 = 11;
        let expected: i32 = 3;

        let result: i32 = Solution::coin_change(coins, amount);

        assert_eq!(expected, result);
    }

    #[test]
    pub fn test_should_pass_basic_case2() {
        let coins: Vec<i32> = vec![2];
        let amount: i32 = 3;
        let expected: i32 = -1;

        let result: i32 = Solution::coin_change(coins, amount);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case3() {
        let coins: Vec<i32> = vec![1];
        let amount: i32 = 0;
        let expected: i32 = 0;

        let result: i32 = Solution::coin_change(coins, amount);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case4() {
        let coins: Vec<i32> = vec![1];
        let amount: i32 = 1;
        let expected: i32 = 1;

        let result: i32 = Solution::coin_change(coins, amount);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case5() {
        let coins: Vec<i32> = vec![1];
        let amount: i32 = 2;
        let expected: i32 = 2;

        let result: i32 = Solution::coin_change(coins, amount);

        assert_eq!(result, expected);
    }
}
