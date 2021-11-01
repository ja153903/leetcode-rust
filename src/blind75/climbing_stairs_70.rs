#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 1;
        let mut b = 1;

        for _ in 1..=n {
            let temp = a;
            a = b;
            b += temp;
        }

        a
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

    #[test]
    pub fn test_should_pass_basic_case2() {
        let n: i32 = 3;
        let expected: i32 = 3;

        let result: i32 = Solution::climb_stairs(n);

        assert_eq!(result, expected);
    }
}
