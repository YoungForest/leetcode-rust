struct Solution {}
use std::collections::HashSet;
impl Solution {
    fn backtracking(
        x: i32,
        y: i32,
        grid: &Vec<Vec<i32>>,
        mut path: &mut HashSet<(i32, i32)>,
    ) -> i32 {
        let next = [(-1, 0), (1, 0), (0, 1), (0, -1)];
        let mut ret = 0;
        for (dx, dy) in next.iter() {
            let nx = x + dx;
            let ny = y + dy;
            match grid.get(nx as usize) {
                Some(row) => match row.get(ny as usize) {
                    None => {
                        continue;
                    }
                    Some(item) => {
                        if *item == 0 || path.contains(&(nx, ny)) {
                            continue;
                        } else {
                            path.insert((nx, ny));
                            let one_way = item + Solution::backtracking(nx, ny, grid, path);
                            ret = std::cmp::max(ret, one_way);
                            path.remove(&(nx, ny));
                        }
                    }
                },
                None => {
                    continue;
                }
            }
        }
        return ret;
    }
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len() as i32;
        let mut n;
        match grid.get(0) {
            Some(value) => {
                n = value.len() as i32;
            }
            None => {
                return 0;
            }
        }
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                let value = grid[i as usize][j as usize];
                if value != 0 {
                    let mut path = HashSet::new();
                    path.insert((i, j));
                    ans =
                        std::cmp::max(ans, value + Solution::backtracking(i, j, &grid, &mut path));
                }
            }
        }
        return ans;
    }
}
pub fn main() {
    println!(
        "{}",
        Solution::get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]])
    );
}
