#![allow(dead_code)]

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::new();

        for num in &nums {
            if set.contains(num) {
                return true;
            }

            set.insert(*num);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_basic_case1() {
        let v: Vec<i32> = vec![1, 3, 2, 1];
        let expected: bool = true;
        let result: bool = Solution::contains_duplicate(v);

        assert_eq!(result, expected);
    }
}
