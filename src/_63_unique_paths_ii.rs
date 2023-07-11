struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
            return 0;
        }

        obstacle_grid[0][0] = 1;

        for y in 0..m {
            for x in 0..n {
                if x == 0 && y == 0 {
                    continue;
                }
                if obstacle_grid[y][x] == 1 {
                    obstacle_grid[y][x] = 0;
                    continue;
                }

                let left_variants = if x == 0 { 0 } else { obstacle_grid[y][x - 1] };
                let top_variants = if y == 0 { 0 } else { obstacle_grid[y - 1][x] };
                obstacle_grid[y][x] = left_variants + top_variants;
            }
        }

        obstacle_grid[m - 1][n - 1]
    }
}

#[cfg(test)]
mod _63_tests {
    use crate::{_63_unique_paths_ii::*, helpers::ConvertableToVecMatrix};

    #[test]
    fn test1() {
        let obstacle_grid = [[0, 0, 0], [0, 1, 0], [0, 0, 0]].to_vecs();
        let ans = 2;

        let res = Solution::unique_paths_with_obstacles(obstacle_grid);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let obstacle_grid = [[0, 1], [0, 0]].to_vecs();
        let ans = 1;

        let res = Solution::unique_paths_with_obstacles(obstacle_grid);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let obstacle_grid = [[0, 1], [1, 0]].to_vecs();
        let ans = 0;

        let res = Solution::unique_paths_with_obstacles(obstacle_grid);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let obstacle_grid = [[0, 0], [0, 1]].to_vecs();
        let ans = 0;

        let res = Solution::unique_paths_with_obstacles(obstacle_grid);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let obstacle_grid = [[1, 0], [0, 0]].to_vecs();
        let ans = 0;

        let res = Solution::unique_paths_with_obstacles(obstacle_grid);

        assert_eq!(res, ans);
    }
}
