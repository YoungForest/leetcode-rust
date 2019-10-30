struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; text1.len() + 1]; text2.len() + 1];
        for i in 1..(text2.len() + 1) {
            for j in 1..(text1.len() + 1) {
                if text1.chars().nth(j - 1).unwrap() == text2.chars().nth(i - 1).unwrap() {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        return dp[text2.len()][text1.len()];
    }
}

pub fn main() {
    println!(
        "{}",
        Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string())
    );
}
