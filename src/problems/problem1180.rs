struct Solution {}
impl Solution {
    pub fn count_letters(s: String) -> i32 {
        let mut ans = 0;
        if s.is_empty() {
            return 0;
        }
        let s = s.into_bytes();
        let mut c = s[0];
        let mut count = 1;
        ans = 1;
        for i in 1..s.len() {
            if c == s[i] {
                count += 1;
            } else {
                count = 1;
            }
            ans += count;
            c = s[i];
        }
        ans
    }
}

pub fn main() {
    println!(
        "{}",
        Solution::count_letters("aaaba".to_string())
    );
}