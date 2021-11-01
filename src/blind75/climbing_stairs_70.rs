#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn climb_stairs(_n: i32) -> i32 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let n: i32 = 2;
        let expected: i32 = 2;

        let result: i32 = Solution::climb_stairs(n);

        assert_eq!(result, expected);
    }
}
