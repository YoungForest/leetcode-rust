mod problems;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    // bfs
    let n = grid.len() as i32;
    assert_eq!(n >= 2, true);
    let m = grid[0].len() as i32;
    assert_eq!(n, m);
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    let mut level = 1;
    q.push_back(((0, 0), (0, 1)));
    seen.insert(((0, 0), (0, 1)));
    while !q.is_empty() {
        let size = q.len();
        for _ in 0..size {
            let current = q.pop_front();
            match current {
                Some(value) => {
                    let next_step: [((i32, i32), (i32, i32)); 4] = [
                        ((0, 1), (0, 1)),
                        ((1, 0), (1, 0)),
                        ((0, 0), (1, -1)),
                        ((0, 0), (-1, 1)),
                    ];
                    for i in 0..4 {
                        let m = next_step[i];
                        let new_tail = (value.0 .0 + m.0 .0, value.0 .1 + m.0 .1);
                        let new_head = (value.1 .0 + m.1 .0, value.1 .1 + m.1 .1);
                        if new_tail.0 < n
                            && new_tail.1 < n
                            && new_head.0 < n
                            && new_head.1 < n
                            && new_tail.0 >= 0
                            && new_tail.1 >= 0
                            && new_head.0 >= 0
                            && new_head.1 >= 0
                            && grid[new_tail.0 as usize][new_tail.1 as usize] == 0
                            && grid[new_head.0 as usize][new_head.1 as usize] == 0
                            && !seen.contains(&(new_tail, new_head))
                        {
                            if i == 0
                                || i == 1
                                || (value.0 .0 + 1 >= 0 // old_tail + 1 is in range
                                        && value.0 .0 + 1 < n
                                        && value.0 .1 + 1 >= 0
                                        && value.0 .1 + 1 < n
                                        && grid[(value.0 .0 + 1) as usize]  // old_tail + 1 is unblock
                                            [(value.0 .1 + 1) as usize]
                                            == 0
                                        && ((i == 2 && value.0 .0 == value.1 .0)
                                            || (i == 3 && value.0 .1 == value.1 .1)))
                            {
                                if new_tail == (n - 1, n - 2) && new_head == (n - 1, n - 1) {
                                    return level;
                                }
                                q.push_back((new_tail, new_head));
                                seen.insert((new_tail, new_head));
                            }
                        }
                    }
                }
                None => (),
            };
        }
        level += 1;
    }
    return -1;
}

pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    let mut qs: HashSet<(i32, i32)> = HashSet::new();
    for queen in queens.iter() {
        qs.insert((queen[0], queen[1]));
    }
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let direction = vec![
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    let k = (king[0], king[1]);
    for (x, y) in direction.iter() {
        let mut target: (i32, i32) = k;
        loop {
            target.0 += x;
            target.1 += y;
            if target.0 >= 0 && target.0 < 8 && target.1 >= 0 && target.1 < 8 {
                if qs.contains(&target) {
                    ans.push(vec![target.0, target.1]);
                    break;
                }
            } else {
                break;
            }
        }
    }
    ans
}

static MOD_NUMBER: i32 = 100000007;

fn answer(
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
                            + answer(&mut memo, &roll_max, n - 1, i, previous_consecutive + 1))
                            % MOD_NUMBER;
                    }
                } else {
                    ans = (ans + answer(&mut memo, &roll_max, n - 1, i, 1)) % MOD_NUMBER;
                }
            }
            memo.insert((n, previous_number, previous_consecutive), ans);
            return ans;
        }
    }
}

pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
    let mut memo: HashMap<(i32, usize, i32), i32> = HashMap::new();
    answer(&mut memo, &roll_max, n, 0, 0)
}

fn main() {
    problems::problem1180::main();
}
