pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        let n = n as usize;
        let mut variants = vec![0; n + 1];

        variants[0] = 1;
        variants[1] = 1;

        for i in 2..=n {
            variants[i] = variants[i - 1] + variants[i - 2];
        }

        variants[n]
    }
}

#[cfg(test)]
mod _70_tests {
    use crate::_70_climing_stairs::*;

    #[test]
    fn test1() {
        let n = 1;
        let ans = 1;

        let res = Solution::climb_stairs(n);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let n = 2;
        let ans = 2;

        let res = Solution::climb_stairs(n);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let n = 3;
        let ans = 3;

        let res = Solution::climb_stairs(n);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let n = 45;
        let ans = 1836311903;

        let res = Solution::climb_stairs(n);

        assert_eq!(res, ans);
    }
}
