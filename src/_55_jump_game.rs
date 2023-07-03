struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_pos = 0;
        let mut pos = 0;

        while pos < nums.len() && pos <= max_pos {
            max_pos = max_pos.max(pos + nums[pos] as usize);
            pos += 1;
        }

        max_pos >= nums.len() - 1
    }
}

#[cfg(test)]
mod _55_tests {
    use crate::_55_jump_game::*;

    #[test]
    fn test1() {
        let nums = vec![2, 3, 1, 1, 4];
        let ans = true;

        let res = Solution::can_jump(nums);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 2, 1, 0, 4];
        let ans = false;

        let res = Solution::can_jump(nums);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let len = 1e4 as usize;
        let mut nums = vec![0; len];
        for (index, el) in nums.iter_mut().enumerate().rev() {
            *el = (len as i32 - index as i32 - 2).max(0);
        }
        let ans = false;

        let res = Solution::can_jump(nums);

        assert_eq!(res, ans);
    }
}
