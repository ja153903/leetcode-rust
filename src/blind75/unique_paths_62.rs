#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // here we can model the state such that
        // dp[i][j] denotes the number of paths up to that point
        // to transition to that state, we would need to know
        // the number of paths coming from above and to the left
        // of that point.
        // this way we can model the state as dp[i][j] = dp[i-1][j] + dp[i][j-1]
        let mut dp: Vec<Vec<i32>> = vec![vec![1; n as usize]; m as usize];

        for i in 1..(m as usize) {
            for j in 1..(n as usize) {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        dp[(m - 1) as usize][(n - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let m: i32 = 3;
        let n: i32 = 7;

        let expected: i32 = 28;

        let result: i32 = Solution::unique_paths(m, n);

        assert_eq!(result, expected);
    }
}
