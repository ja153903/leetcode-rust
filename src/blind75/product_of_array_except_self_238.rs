#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // [1, 2, 3, 4]
        // [1, 1, 1, 1] is initial state
        // [1, 1, 2, 6] is what we get when we look forward and multiply previous elements
        // [24, 12, 8, 6] is what we get after we go backwards
        let mut result: Vec<i32> = vec![1; nums.len()];

        for i in 1..nums.len() {
            result[i] = result[i-1] * nums[i-1];
        }

        let mut build_up: i32 = 1;
        for i in (0..nums.len()).rev() {
            result[i] *= build_up;
            build_up *= nums[i];
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let v: Vec<i32> = vec![1,2,3,4];
        let expected: Vec<i32> = vec![24, 12, 8, 6];

        let result: Vec<i32> = Solution::product_except_self(v);

        assert_eq!(result, expected);
    }
}
