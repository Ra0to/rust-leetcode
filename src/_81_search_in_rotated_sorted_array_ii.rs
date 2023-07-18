pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let rotation_point = nums
            .windows(2)
            .enumerate()
            .find(|(_, x)| x[1] < x[0])
            .map(|(ind, _)| ind);

        match rotation_point {
            Some(rotation_index) if target <= nums[rotation_index] && target >= nums[0] => {
                nums[..=rotation_index].binary_search(&target).is_ok()
            }
            Some(rotation_index) => nums[rotation_index + 1..].binary_search(&target).is_ok(),
            None => nums.binary_search(&target).is_ok(),
        }
    }
}

#[cfg(test)]
mod _81_search_in_rotated_sorted_array_ii {
    use crate::_81_search_in_rotated_sorted_array_ii::*;

    #[test]
    fn test1() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 0;
        let ans = true;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 3;
        let ans = false;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let nums = vec![0];
        let target = 0;
        let ans = true;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let nums = vec![0];
        let target = 1;
        let ans = false;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let nums = vec![0, 0, 1, 2, 2, 5, 6];
        let target = 3;
        let ans = false;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test6() {
        let nums = vec![0, 0, 1, 2, 2, 5, 6];
        let target = 0;
        let ans = true;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test7() {
        let nums = vec![0, 1, 2, 3];
        let target = 0;
        let ans = true;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test8() {
        let nums = vec![1, 2, 3, 0];
        let target = 0;
        let ans = true;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test9() {
        let nums = vec![2, 3, 0, 1];
        let target = 0;
        let ans = true;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test10() {
        let nums = vec![3, 0, 1, 2];
        let target = 0;
        let ans = true;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test11() {
        let nums = vec![0, 1, 2, 3];
        let target = 4;
        let ans = false;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test12() {
        let nums = vec![1, 2, 3, 0];
        let target = 4;
        let ans = false;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test13() {
        let nums = vec![2, 3, 0, 1];
        let target = 4;
        let ans = false;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test14() {
        let nums = vec![3, 0, 1, 2];
        let target = 4;
        let ans = false;

        let res = Solution::search(nums, target);

        assert_eq!(res, ans);
    }
}
