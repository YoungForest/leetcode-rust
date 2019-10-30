struct Solution {
}
use std::collections::HashMap;

impl Solution {
    const MOD_NUMBER: i32 = 1000000007;

    pub fn answer(
        mut memo: &mut HashMap<(i32, usize, i32), i32>,
        roll_max: &Vec<i32>,
        n: i32,
        previous_number: usize,
        previous_consecutive: i32,
    ) -> i32 {
        if n == 0 {
            return 1;
        }
        match memo.get(&(n, previous_number, previous_consecutive)) {
            Some(value) => {
                return *value;
            }
            None => {
                let mut ans = 0;
                for i in 0..6 {
                    if i == previous_number {
                        if previous_consecutive == roll_max[previous_number] {
                            continue;
                        } else {
                            ans = (ans
                                + Solution::answer(&mut memo, &roll_max, n - 1, i, previous_consecutive + 1))
                                % Solution::MOD_NUMBER;
                        }
                    } else {
                        ans = (ans + Solution::answer(&mut memo, &roll_max, n - 1, i, 1)) % Solution::MOD_NUMBER;
                    }
                }
                memo.insert((n, previous_number, previous_consecutive), ans);
                return ans;
            }
        }
    }
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let mut memo: HashMap<(i32, usize, i32), i32> = HashMap::new();
        Solution::answer(&mut memo, &roll_max, n, 0, 0)
    }
}
pub fn main() {
    println!("{}", Solution::die_simulator(2, vec![1, 1, 2, 2, 2, 3]));
}
