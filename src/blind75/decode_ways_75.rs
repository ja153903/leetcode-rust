#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut dp: Vec<i32> = vec![0; s.len() + 1];
        dp[0] = 1;
        dp[1] = if &s[0..1] != "0" { 1 } else { 0 };

        for i in 2..=s.len() {
            let n1: i32 = s[(i - 1)..i].parse::<i32>().expect("something went wrong");
            let n2: i32 = s[(i - 2)..i].parse::<i32>().expect("something went wrong");

            if (1..=9).contains(&n1) {
                dp[i] += dp[i - 1];
            }

            if (10..=26).contains(&n2) {
                dp[i] += dp[i - 2];
            }
        }

        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let s = String::from("12");
        let expected: i32 = 2;

        let result: i32 = Solution::num_decodings(s);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case2() {
        let s = String::from("226");
        let expected: i32 = 3;

        let result: i32 = Solution::num_decodings(s);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case3() {
        let s = String::from("0");
        let expected: i32 = 0;

        let result: i32 = Solution::num_decodings(s);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case4() {
        let s = String::from("06");
        let expected: i32 = 0;

        let result: i32 = Solution::num_decodings(s);

        assert_eq!(result, expected);
    }
}
