
cat > $1 << EOL
#![allow(dead_code)]

struct Solution;

impl Solution {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_should_pass_basic_case1() {}
}
EOL
