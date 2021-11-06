#![allow(dead_code)]

struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // this is a dynamic programming question
        // if we model the state, we would model it as follows:
        // let dp[i] determine whether we can create a string that is contained within the hashset up to this point
        let mut dp: Vec<bool> = vec![false; s.len() + 1];
        let hashset: HashSet<String> = HashSet::from_iter(word_dict.iter().cloned());

        dp[0] = true;

        for i in 1..=s.len() {
            // for each i, we would have to look back to see if we can create some word within the
            // word_dict up to this point
            for j in 0..i {
                // if the current index is true, then this means that it was a former stopping
                // point for some word that is contained within the word_dict
                // if this is the case, then we check if the current substring is contained within
                // the set, if it is, then we can mark index i as true
                if dp[j] && hashset.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {
        let s: String = String::from("leetcode");
        let word_dict: Vec<String> = vec![String::from("leet"), String::from("code")];

        let expected: bool = true;
        let result: bool = Solution::word_break(s, word_dict);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case2() {
        let s: String = String::from("applepenapple");
        let word_dict: Vec<String> = vec![String::from("apple"), String::from("pen")];

        let expected: bool = true;
        let result: bool = Solution::word_break(s, word_dict);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_should_pass_basic_case3() {
        let s: String = String::from("catsandog");
        let word_dict: Vec<String> = vec![
            String::from("cats"),
            String::from("dogs"),
            String::from("sand"),
            String::from("and"),
            String::from("cat"),
        ];

        let expected: bool = false;
        let result: bool = Solution::word_break(s, word_dict);

        assert_eq!(result, expected);
    }
}
