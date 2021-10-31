#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn hamming_weight(mut n: u32) -> i32 {
        let mut count: i32 = 0;

        while n != 0 {
            // increase count if last bit is a 1
            count = count + (n & 1) as i32;
            n = n >> 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_basic_case1() {
        let n: u32 = 11;
        let expected: i32 = 3;
        let result: i32 = Solution::hamming_weight(n);

        assert_eq!(result, expected);
    }
}
