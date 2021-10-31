#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn missing_number(_nums: Vec<i32>) -> i32 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let nums: Vec<i32> = vec![3, 0, 1];
        let expected: i32 = 2;

        let result = Solution::missing_number(nums);

        assert_eq!(result, expected);
    }
}
