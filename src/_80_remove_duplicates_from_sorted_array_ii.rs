pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut cnt = 2 as usize;
        let mut last = nums.len() - 1;

        let mut ind = 2 as usize;

        dbg!(&nums);

        while ind <= last {
            if nums[ind] != nums[ind - 1] || nums[ind] != nums[ind - 2] {
                ind += 1;
                cnt += 1;
                continue;
            }

            println!("move from {ind} to {last}");
            for swap_ind in ind..last {
                nums.swap(swap_ind, swap_ind + 1);
            }

            last -= 1;
        }

        dbg!(&nums[..=last]);

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
}
