#![allow(dead_code, unused_comparisons)]

struct Solution;

impl Solution {
    /**
     * longest_palindrome(s)
     *      for each index of the string, expand around it by creating two iterating pointers (i, j)
     *          while i >= 0 and j < len(s)
     *              check if characters at i and j match:
     *                  if they do not match: stop iterating and return previous max palindrome
     *                  else: decrement i and increment j
     */
    pub fn longest_palindrome(s: String) -> String {
        let mut i: i32 = 0;
        let mut current_string: String = String::new();

        while i < s.len() as i32 {
            let (palindrome1, len1) = Solution::get_palindrome(s.as_str(), i, i);
            let (palindrome2, len2) = Solution::get_palindrome(s.as_str(), i, i + 1);

            let current_max_string = if len1 < len2 {
                palindrome2
            } else {
                palindrome1
            };

            current_string = if current_string.len() < current_max_string.len() {
                current_max_string
            } else {
                current_string
            };

            i += 1;
        }

        current_string
    }

    fn get_palindrome(s: &str, mut i: i32, mut j: i32) -> (String, i32) {
        let mut palindrome: String = s[(i as usize)..(i as usize) + 1].to_string();

        while i >= 0 && j < s.len() as i32 {
            let si = i as usize;
            let sj = j as usize;

            if s[si..si + 1] == s[sj..sj + 1] {
                palindrome = s[si..=sj].to_string();
                i -= 1;
                j += 1;
            } else {
                break;
            }
        }

        (palindrome, j - i + 1)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn test_basic_case1() {
        let s = String::from("babad");
        let expected = String::from("bab");
        let result = Solution::longest_palindrome(s);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_basic_case2() {
        let s = String::from("cbbd");
        let expected = String::from("bb");
        let result = Solution::longest_palindrome(s);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_basic_case3() {
        let s = String::from("a");
        let expected = String::from("a");
        let result = Solution::longest_palindrome(s);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_basic_case4() {
        let s = String::from("ab");
        let expected = String::from("a");
        let result = Solution::longest_palindrome(s);

        assert_eq!(result, expected);
    }
}
