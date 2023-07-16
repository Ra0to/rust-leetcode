pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut cnt = 2;

        for ind in 2..nums.len() {
            if nums[ind] == nums[cnt - 1] && nums[ind] == nums[cnt - 2] {
                continue;
            }

            nums.swap(ind, cnt);
            cnt += 1;
        }

        cnt as i32
    }
}

#[cfg(test)]
mod _80_tests {
    use crate::_80_remove_duplicates_from_sorted_array_ii::*;

    #[test]
    fn test1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let ans_len = 5;
        let ans = vec![1, 1, 2, 2, 3];

        let res = Solution::remove_duplicates(&mut nums);

        assert_eq!(res, ans_len);
        assert_eq!(nums[..res as usize], ans);
    }

    #[test]
    fn test2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let ans_len = 7;
        let ans = vec![0, 0, 1, 1, 2, 3, 3];

        let res = Solution::remove_duplicates(&mut nums);

        assert_eq!(res, ans_len);
        assert_eq!(nums[..res as usize], ans);
    }

    #[test]
    fn test3() {
        let mut nums = vec![0];
        let ans_len = 1;
        let ans = vec![0];

        let res = Solution::remove_duplicates(&mut nums);

        assert_eq!(res, ans_len);
        assert_eq!(nums[..res as usize], ans);
    }

    #[test]
    fn test4() {
        let mut nums = vec![0, 0, 0, 0, 0];
        let ans_len = 2;
        let ans = vec![0, 0];

        let res = Solution::remove_duplicates(&mut nums);

        assert_eq!(res, ans_len);
        assert_eq!(nums[..res as usize], ans);
    }

    #[test]
    fn test5() {
        let mut nums = vec![0, 1, 2, 3, 4];
        let ans_len = 5;
        let ans = vec![0, 1, 2, 3, 4];

        let res = Solution::remove_duplicates(&mut nums);

        assert_eq!(res, ans_len);
        assert_eq!(nums[..res as usize], ans);
    }
}
