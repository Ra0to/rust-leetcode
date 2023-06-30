struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut index: usize = 0;
        let len = nums.len();

        // Place each number on their position
        // E.g. 1 place is 0
        // Skip negatives and zero
        while index < len {
            let x = nums[index];
            if x <= 0 {
                index += 1;
                continue;
            }

            if x >= len as i32 {
                index += 1;
                continue;
            }

            if x - 1 == index as i32 {
                index += 1;
                continue;
            }

            if x == nums[(x - 1) as usize] {
                index += 1;
                continue;
            }

            nums.swap((x - 1) as usize, index);
        }

        // Find first index where number doesn't match index
        index = 0;
        while index < len {
            if nums[index] - 1 != index as i32 {
                return (index + 1) as i32;
            }

            index += 1;
        }

        return (len + 1) as i32;
    }
}

#[cfg(test)]
mod _41_tests {
    use crate::_41_first_missing_positive::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 0];
        let ans = 3;

        let res = Solution::first_missing_positive(nums);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 4, -1, 1];
        let ans = 2;

        let res = Solution::first_missing_positive(nums);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let nums = vec![7, 8, 9, 11, 12];
        let ans = 1;

        let res = Solution::first_missing_positive(nums);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let nums = vec![1];
        let ans = 2;

        let res = Solution::first_missing_positive(nums);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let nums = vec![-1];
        let ans = 1;

        let res = Solution::first_missing_positive(nums);

        assert_eq!(res, ans);
    }

    #[test]
    fn test6() {
        let nums = vec![1, 2, 3];
        let ans = 4;

        let res = Solution::first_missing_positive(nums);

        assert_eq!(res, ans);
    }
}
