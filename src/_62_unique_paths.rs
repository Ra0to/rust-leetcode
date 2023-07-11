pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut grid = vec![vec![0; n]; m];

        for i in 0..n.max(m) {
            if i < m {
                grid[i][0] = 1;
            }
            if i < n {
                grid[0][i] = 1;
            }
        }

        for y in 1..m {
            for x in 1..n {
                grid[y][x] = grid[y - 1][x] + grid[y][x - 1];
            }
        }

        grid[m - 1][n - 1]
    }
}

#[cfg(test)]
mod _62_tests {
    use crate::_62_unique_paths::*;

    #[test]
    fn test1() {
        let m = 3;
        let n = 7;
        let ans = 28;

        let res = Solution::unique_paths(m, n);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let m = 3;
        let n = 2;
        let ans = 3;

        let res = Solution::unique_paths(m, n);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let m = 1;
        let n = 1;
        let ans = 1;

        let res = Solution::unique_paths(m, n);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let m = 50;
        let n = 9;
        let ans: i32 = 1652411475;

        let res = Solution::unique_paths(m, n);

        assert_eq!(res, ans);
    }
}
