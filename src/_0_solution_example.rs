pub struct Solution;

impl Solution {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

#[cfg(test)]
mod _0_tests {
    use crate::_0_solution_example::*;

    #[test]
    fn test1() {
        let x = 0;
        let y = 0;
        let ans = 0;

        let res = Solution::add(x, y);

        assert_eq!(res, ans);
    }
}
