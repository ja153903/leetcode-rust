#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut power: i32 = 31;
        let mut rev: u32 = 0;

        while x > 0 {
            // we compute the last bit and left shift by the power so that we get the appropriate
            // spot for this bit.
            // then we add it to the result since this is basically appending it
            // e.x. suppose rev = 010 then we want to add 100, it becomes 110
            rev += (x & 1) << power;

            // right shift to get the next bit
            x >>= 1;

            power -= 1;
        }

        rev
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let n: u32 = 4294967293;
        let expected: u32 = 3221225471;

        let result: u32 = Solution::reverse_bits(n);

        assert_eq!(result, expected);
    }
}
