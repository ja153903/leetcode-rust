#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        nums.sort();

        let mut i: i32 = 0;

        while i < (nums.len() as i32) - 2 {
            if i == 0 || (i > 0 && nums[i as usize] != nums[(i - 1) as usize]) {
                let mut j: i32 = i + 1;
                let mut k: i32 = (nums.len() as i32) - 1;

                while j < k {
                    let target: i32 = -nums[i as usize];
                    let current: i32 = nums[j as usize] + nums[k as usize];

                    if current == target {
                        result.push(vec![nums[i as usize], nums[j as usize], nums[k as usize]]);

                        while j < k && nums[j as usize] == nums[(j + 1) as usize] {
                            j += 1;
                        }
                        while j < k && nums[k as usize] == nums[(k - 1) as usize] {
                            k -= 1;
                        }

                        j += 1;
                        k -= 1;
                    } else if current < target {
                        j += 1;
                    } else {
                        k -= 1;
                    }
                }
            }

            i += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_basic_case1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

        let result = Solution::three_sum(nums);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_basic_case2() {
        let nums = vec![];
        let expected: Vec<Vec<i32>> = vec![];

        let result = Solution::three_sum(nums);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_basic_case3() {
        let nums = vec![0];
        let expected: Vec<Vec<i32>> = vec![];

        let result = Solution::three_sum(nums);

        assert_eq!(result, expected);
    }
}
