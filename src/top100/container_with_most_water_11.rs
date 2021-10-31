#![allow(dead_code)]

struct Solution;

use std::cmp;

impl Solution {
    /*
     * max_area(heights)
     *      we can start at the two ends. and compare the areas at those points
     *      the area can be calculated as min(height[i], height[j]) * (j - i)
     *      we do this as long as i < j
     */
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i: usize = 0;
        let mut j: usize = height.len() - 1;
        let mut max_area: i32 = 0;

        while i < j {
            let current_area = cmp::min(height[i], height[j]) * (j - i) as i32;
            max_area = cmp::max(max_area, current_area);

            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_basic_case1() {
        let height = vec![1, 1];
        let expected = 1;

        let result = Solution::max_area(height);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn should_pass_basic_case2() {
        let height = vec![4, 3, 2, 1, 4];
        let expected = 16;

        let result = Solution::max_area(height);

        assert_eq!(result, expected);
    }
}
