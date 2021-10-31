#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        // find the index of the smallest number
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;

        while left < right {
            let mid = (left + right) / 2;

            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_basic_case1() {
        let nums: Vec<i32> = vec![3, 4, 5, 1, 2];
        let expected: i32 = 1;

        let result: i32 = Solution::find_min(nums);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_basic_case2() {
        let nums: Vec<i32> = vec![4, 5, 6, 7, 0, 1, 2];
        let expected: i32 = 0;

        let result: i32 = Solution::find_min(nums);

        assert_eq!(result, expected);
    }
}
