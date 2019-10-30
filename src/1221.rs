impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut ans = 0;
        let mut l = 0;
        let mut r = 0;
        for c in s.chars() {
            match c {
                'L' => { l += 1},
                'R' => { r += 1},
                _ => (),
            }
            if l == r {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    println!("{}", balanced_string_split("LLLLRRRR"));
}
