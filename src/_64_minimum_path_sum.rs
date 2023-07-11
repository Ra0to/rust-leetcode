pub struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        for y in 0..m {
            for x in 0..n {
                if x == 0 && y == 0 {
                    continue;
                }

                let sum_path_from_top = if y == 0 { i32::MAX } else { grid[y - 1][x] };
                let sum_path_from_left = if x == 0 { i32::MAX } else { grid[y][x - 1] };
                grid[y][x] += i32::min(sum_path_from_top, sum_path_from_left);
            }
        }

        grid[m - 1][n - 1]
    }
}

#[cfg(test)]
mod _64_tests {
    use crate::{_64_minimum_path_sum::*, helpers::ConvertableToVecMatrix};

    #[test]
    fn test1() {
        let grid = [[1, 3, 1], [1, 5, 1], [4, 2, 1]].to_vecs();
        let ans = 7;

        let res = Solution::min_path_sum(grid);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let grid = [[1, 2, 3], [4, 5, 6]].to_vecs();
        let ans = 12;

        let res = Solution::min_path_sum(grid);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let grid = [[1]].to_vecs();
        let ans = 1;

        let res = Solution::min_path_sum(grid);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let grid = [[10]].to_vecs();
        let ans = 10;

        let res = Solution::min_path_sum(grid);

        assert_eq!(res, ans);
    }
}
