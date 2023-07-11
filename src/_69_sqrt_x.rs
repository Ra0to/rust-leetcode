pub struct Solution;

const SQRT_MAX: i32 = 46340;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 1 {
            return 1;
        }

        let mut l = 0;
        let mut r = x;

        while r - l > 1 {
            let m = (r + l) / 2;

            if m > SQRT_MAX || m * m > x {
                r = m;
            } else {
                l = m;
            }
        }

        l
    }
}

#[cfg(test)]
mod _69_tests {
    use crate::_69_sqrt_x::*;

    #[test]
    fn test1() {
        let x = 4;
        let ans = 2;

        let res = Solution::my_sqrt(x);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let x = 8;
        let ans = 2;

        let res = Solution::my_sqrt(x);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let x = 2147483647;
        let ans = 46340;

        let res = Solution::my_sqrt(x);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let x = 1;
        let ans = 1;

        let res = Solution::my_sqrt(x);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let x = 0;
        let ans = 0;

        let res = Solution::my_sqrt(x);

        assert_eq!(res, ans);
    }
}
