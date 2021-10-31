#![allow(dead_code)]

struct Solution;

use std::cmp;

impl Solution {
    pub fn brute_force(nums: Vec<i32>) -> i32 {
        let mut max: i32 = 0;

        for i in 0..nums.len() {
            let mut current_prod: i32 = 1;
            let mut current_max: i32 = 0;

            for &num in nums.iter().skip(i) {
                current_prod *= num;
                current_max = cmp::max(current_max, current_prod);
            }

            max = cmp::max(max, current_max);
        }

        max
    }

    // This problem is trivial if we have all positive integers
    // since if that's the case, then the product of the entire vector will
    // be the max product.
    // Suppose we think about this problem recursively.
    // if the length of the array is 1, which is our base case, then we return whatever that
    // value is. Note that the max value cannot actually be 0 since the vector could contain
    // just a negative integer
    // then if the length of the array is greater than 1, we do two recursive calls.
    // One call should multiply the current index i with the rest of the recursive calls
    // and the other call should skip the current index and find the max in the rest of the other
    // calls starting with i + 1
    // it would be implemented as follows
    pub fn recursive(nums: Vec<i32>, start: usize) -> i32 {
        if nums.len() == 1 || start == nums.len() - 1 {
            nums[start]
        } else {
            let mut current_max: i32 = 0;
            let mut current_prod: i32 = nums[start];

            for &num in nums.iter().skip(start + 1) {
                current_prod *= num;
                current_max = cmp::max(current_max, current_prod);
            }
            
            cmp::max(current_max, Solution::recursive(nums, start + 1))
        }
    }

    // This problem ends up being a variant of maximum subarray
    // where we also keep track of the min
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut max_so_far = nums[0];
        let mut min_so_far = nums[0];
        let mut current_max = nums[0];

        for &num in nums.iter().skip(1) {
            let temp_max = cmp::max(num, cmp::max(num * max_so_far, num * min_so_far));
            min_so_far = cmp::min(num, cmp::min(num * max_so_far, num * min_so_far));

            max_so_far = temp_max;

            current_max = cmp::max(current_max, max_so_far);
        }

        current_max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_test_with_recursive() {
        let nums: Vec<i32> = vec![2, 3, -2, 4];
        let expected: i32 = 6;

        let result: i32 = Solution::recursive(nums, 0);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_test_with_brute_force() {
        let nums: Vec<i32> = vec![2, 3, -2, 4];
        let expected: i32 = 6;

        let result: i32 = Solution::brute_force(nums);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case1() {
        let nums: Vec<i32> = vec![2, 3, -2, 4];
        let expected: i32 = 6;

        let result: i32 = Solution::max_product(nums);

        assert_eq!(result, expected);
    }
}
