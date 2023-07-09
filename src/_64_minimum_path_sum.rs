struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut min_path = vec![vec![0; n]; m];

        for y in 0..m {
            for x in 0..n {
                if x == 0 && y == 0 {
                    continue;
                }

                let sum_path_from_top = if y == 0 {
                    i32::MAX
                } else {
                    min_path[y - 1][x] + grid[y - 1][x]
                };
                let sum_path_from_left = if x == 0 {
                    i32::MAX
                } else {
                    min_path[y][x - 1] + grid[y][x - 1]
                };
                min_path[y][x] = i32::min(sum_path_from_top, sum_path_from_left);
            }
        }

        min_path[m - 1][n - 1] + grid[m - 1][n - 1]
    }
}

#[cfg(test)]
mod _64_tests {
    use crate::_64_minimum_path_sum::*;

    #[test]
    fn test1() {
        let grid = [[1, 3, 1], [1, 5, 1], [4, 2, 1]]
            .into_iter()
            .map(|x| Vec::from_iter(x))
            .collect::<Vec<Vec<i32>>>();
        let ans = 7;

        let res = Solution::min_path_sum(grid);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let grid = [[1, 2, 3], [4, 5, 6]]
            .into_iter()
            .map(|x| Vec::from_iter(x))
            .collect::<Vec<Vec<i32>>>();
        let ans = 12;

        let res = Solution::min_path_sum(grid);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let grid = [[1]]
            .into_iter()
            .map(|x| Vec::from_iter(x))
            .collect::<Vec<Vec<i32>>>();
        let ans = 1;

        let res = Solution::min_path_sum(grid);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let grid = [[10]]
            .into_iter()
            .map(|x| Vec::from_iter(x))
            .collect::<Vec<Vec<i32>>>();
        let ans = 10;

        let res = Solution::min_path_sum(grid);

        assert_eq!(res, ans);
    }
}
