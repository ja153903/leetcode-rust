#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut dp: Vec<i32> = vec![0; (n + 1) as usize];

        for i in 1..=n {
            // this bit trick drops the last bit set
            let last_bit_set = i & (i - 1);

            // so if we get the index of the last bit set
            // that's the number of 1s that we need to add from
            // and obviously since we're setting a bit at i
            // we will need to add 1
            dp[i as usize] = dp[last_bit_set as usize] + 1;
        }

        dp
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_basic_case1() {
        let n: i32 = 2;
        let expected: Vec<i32> = vec![0, 1, 1];

        let result: Vec<i32> = Solution::count_bits(n);

        assert_eq!(result, expected);
    }
}
