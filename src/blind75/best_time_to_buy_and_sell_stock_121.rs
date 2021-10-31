#![allow(dead_code)]

struct Solution;

use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_buy: i32 = prices[0];
        let mut profit: i32 = 0;

        for price in &prices {
            min_buy = cmp::min(min_buy, *price);
            profit = cmp::max(profit, *price - min_buy);
        }

        profit
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let v: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
        let expected: i32 = 5;
        let result = Solution::max_profit(v);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case2() {
        let v: Vec<i32> = vec![7, 6, 4, 3, 1];
        let expected: i32 = 0;
        let result = Solution::max_profit(v);

        assert_eq!(result, expected);
    }
}
