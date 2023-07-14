pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let mut max_point_we_can_reach = nums[0] as usize;
        let mut jumps = 1;
        let mut index = 0;

        while max_point_we_can_reach < nums.len() - 1 {
            for i in index..=max_point_we_can_reach {
                max_point_we_can_reach =
                    std::cmp::max(max_point_we_can_reach, nums[i] as usize + i);
            }

            index = max_point_we_can_reach;
            jumps += 1;
        }

        return jumps;
    }
}

#[cfg(test)]
mod _45_tests {
    use crate::_45_jump_game_ii::*;

    #[test]
    fn test1() {
        let nums = vec![2, 3, 1, 1, 4];
        let ans = 2;

        let res = Solution::jump(nums);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 3, 0, 1, 4];
        let ans = 2;

        let res = Solution::jump(nums);

        assert_eq!(res, ans);
    }
}
