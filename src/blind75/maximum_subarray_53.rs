#![allow(dead_code)]

struct Solution;

use std::cmp;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_so_far = nums[0];
        let mut current_max = nums[0];

        for &num in nums.iter().skip(1) {
            max_so_far = cmp::max(num, num + max_so_far);
            current_max = cmp::max(current_max, max_so_far);
        }

        current_max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let v: Vec<i32> = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let expected: i32 = 6;
        let result = Solution::max_sub_array(v);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case2() {
        let v: Vec<i32> = vec![1];
        let expected: i32 = 1;
        let result = Solution::max_sub_array(v);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case3() {
        let v: Vec<i32> = vec![5, 4, -1, 7, 8];
        let expected: i32 = 23;
        let result = Solution::max_sub_array(v);

        assert_eq!(result, expected);
    }
}
