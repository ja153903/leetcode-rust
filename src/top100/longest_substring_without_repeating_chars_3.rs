#![allow(dead_code)]

struct Solution;

use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen = HashMap::new();
        let mut max = 0;
        let mut start = 0;

        for (current, ch) in s.chars().enumerate() {
            if last_seen.contains_key(&ch) {
                let last_occurence = last_seen.get(&ch).unwrap();
                // we update the pointer based on the current max of the start pointer
                // i.e. look at the string "abba"
                // if we just update based on last occurence, we'll get a length of 3
                // rather than 2. We have to make sure that we calculate based on new start pointer
                start = cmp::max(start, *last_occurence + 1);
            }

            max = cmp::max(max, current - start + 1);
            last_seen.insert(ch, current);
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_basic_case1() {
        let s = String::from("abcabcbb");
        let result = Solution::length_of_longest_substring(s);
        let expected = 3;

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_basic_case2() {
        let s = String::from("bbbbb");
        let result = Solution::length_of_longest_substring(s);
        let expected = 1;

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_basic_case3() {
        let s = String::from("pwwkew");
        let result = Solution::length_of_longest_substring(s);
        let expected = 3;

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_basic_case4() {
        let s = String::from("");
        let result = Solution::length_of_longest_substring(s);
        let expected = 0;

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_basic_case5() {
        let s = String::from("dvdf");
        let result = Solution::length_of_longest_substring(s);
        let expected = 3;

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_edge_case() {
        let s = String::from("abba");
        let result = Solution::length_of_longest_substring(s);
        let expected = 2;

        assert_eq!(result, expected);
    }
}
