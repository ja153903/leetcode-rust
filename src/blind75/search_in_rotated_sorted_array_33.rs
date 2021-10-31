#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        if nums.len() == 1 {
            return if nums[0] == target { 0 } else { -1 };
        }

        // find the minimum in the array this will serve as the pivot index

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

        let pivot: usize = left;

        left = 0;
        right = nums.len() - 1;

        while left <= right {
            let mid: usize = (left + right) / 2;
            let true_mid = (mid + pivot) % nums.len();

            if nums[true_mid] == target {
                return true_mid as i32;
            } else if nums[true_mid] < target {
                left = mid + 1;
            } else if mid == 0{
                break;
            } else {
                right = mid - 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_basic_case1() {
        let nums: Vec<i32> = vec![4, 5, 6, 7, 0, 1, 2];
        let target: i32 = 0;
        let expected: i32 = 4;

        let result: i32 = Solution::search(nums, target);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_basic_case2() {
        let nums: Vec<i32> = vec![4, 5, 6, 7, 0, 1, 2];
        let target: i32 = 3;
        let expected: i32 = -1;

        let result: i32 = Solution::search(nums, target);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_basic_case3() {
        let nums: Vec<i32> = vec![1, 3];
        let target: i32 = 0;
        let expected: i32 = -1;

        let result: i32 = Solution::search(nums, target);

        assert_eq!(result, expected);
    }
}
