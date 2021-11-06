#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; text2.len() + 1]; text1.len() + 1];

        for i in 1..=text1.len() {
            for j in 1..=text2.len() {
                dp[i][j] = if text1[i - 1..i] == text2[j - 1..j] {
                    dp[i - 1][j - 1] + 1
                } else {
                    std::cmp::max(std::cmp::max(dp[i][j - 1], dp[i - 1][j]), dp[i - 1][j - 1])
                }
            }
        }

        dp[text1.len()][text2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let s1: String = String::from("abcde");
        let s2: String = String::from("ace");

        let expected: i32 = 3;

        let result: i32 = Solution::longest_common_subsequence(s1, s2);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case2() {
        let s1: String = String::from("abc");
        let s2: String = String::from("abc");

        let expected: i32 = 3;

        let result: i32 = Solution::longest_common_subsequence(s1, s2);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case3() {
        let s1: String = String::from("abc");
        let s2: String = String::from("def");

        let expected: i32 = 0;

        let result: i32 = Solution::longest_common_subsequence(s1, s2);

        assert_eq!(result, expected);
    }
}
