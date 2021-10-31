#![allow(dead_code)]

mod binary_search {
    pub fn search_minimum_in_rotated_vector<T: PartialOrd + PartialEq>(
        nums: Vec<T>,
    ) -> Option<usize> {
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;

        while left < right {
            let mid: usize = (left + right) / 2;

            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        Some(left)
    }

    pub fn search_rotated_vector<T: PartialOrd + PartialEq>(
        nums: Vec<T>,
        target: T,
        pivot: usize,
    ) -> Option<usize> {
        let n = nums.len();
        let mut left: usize = 0;
        let mut right: usize = n - 1;

        while left <= right {
            let mid: usize = (left + right) / 2;
            let true_mid = (mid + pivot) % n;

            if nums[true_mid] == target {
                return Some(true_mid);
            } else if nums[true_mid] > target {
                right = mid - 1;
            } else {
                left = mid - 1;
            }
        }

        None
    }
}
