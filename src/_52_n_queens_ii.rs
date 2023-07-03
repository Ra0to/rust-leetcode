struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        crate::_51_n_queens::Solution::solve_n_queens(n).len() as i32
    }
}

#[cfg(test)]
mod _52_tests {
    use crate::_52_n_queens_ii::*;

    #[test]
    fn test1() {
        let n = 1;
        let ans = 1;

        let res = Solution::total_n_queens(n);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let n = 2;
        let ans = 0;

        let res = Solution::total_n_queens(n);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let n = 4;
        let ans = 2;

        let res = Solution::total_n_queens(n);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let n = 9;
        let ans = 352;

        let res = Solution::total_n_queens(n);

        assert_eq!(res, ans);
    }
}
