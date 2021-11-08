#![allow(dead_code)]

struct Solution;

// You are given an integer array nums. You are initially positioned at the array's first index,
// and each element in the array represents your maximum jump length at that position.
// Return true if you can reach the last index, or false otherwise.

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.is_empty() || nums.len() == 1 {
            return true;
        }

        // the idea with this solution is to keep track of the smallest index that can jump to the
        // end
        let mut last: usize = nums.len() - 1;

        for i in (0..=nums.len() - 2).rev() {
            if i + nums[i] as usize >= last {
                last = i;
            }
        }

        last == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let nums: Vec<i32> = vec![2, 3, 1, 1, 4];
        let expected: bool = true;

        let result: bool = Solution::can_jump(nums);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case2() {
        let nums: Vec<i32> = vec![3, 2, 1, 0, 4];
        let expected: bool = false;

        let result: bool = Solution::can_jump(nums);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case3() {
        let nums: Vec<i32> = vec![0];
        let expected: bool = true;

        let result: bool = Solution::can_jump(nums);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case4() {
        let nums: Vec<i32> = vec![0, 2, 3];
        let expected: bool = false;

        let result: bool = Solution::can_jump(nums);

        assert_eq!(result, expected);
    }
}
