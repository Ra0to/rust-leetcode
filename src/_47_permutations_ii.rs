struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::<Vec<i32>>::new();
        // nums.len <= 8
        let is_used: i8 = 0b0000_0000;
        let mut current = vec![0; nums.len()];

        // I can sort nums and then skip same numbers
        // E.g. we have nums = [1, 2, 1]
        // Idea #1 permute(nums) == permute(nums.sort())
        // Idea #2 if I've already checked first 1 in [1, 1, 2], I can skip all other ones
        // Because I've already add solutions for case where 1 is on first place.

        nums.sort_unstable();
        Self::fill_answers(&nums, is_used, &mut current, 0, &mut ans);
        ans
    }

    fn fill_answers(
        nums: &Vec<i32>,
        is_used: i8,
        mut current: &mut Vec<i32>,
        index: usize,
        mut ans: &mut Vec<Vec<i32>>,
    ) {
        if index >= nums.len() {
            ans.push(current.clone());
            return;
        }

        let mut prev: Option<i32> = None;

        for (i, num) in nums.iter().enumerate() {
            // We've already checked this variants
            if prev == Some(*num) {
                continue;
            }

            let flag = 1 << i;
            if is_used & flag != 0 {
                continue;
            }

            current[index] = *num;
            Self::fill_answers(&nums, is_used | flag, &mut current, index + 1, &mut ans);
            prev = Some(*num);
        }
    }
}

#[cfg(test)]
mod _47_tests {
    use crate::_47_permutations_ii::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3];
        let mut ans = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];

        let mut res = Solution::permute_unique(nums);

        ans.sort_unstable();
        res.sort_unstable();

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 1];
        let mut ans = vec![vec![0, 1], vec![1, 0]];

        let mut res = Solution::permute_unique(nums);

        ans.sort_unstable();
        res.sort_unstable();

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let nums = vec![1];
        let mut ans = vec![vec![1]];

        let mut res = Solution::permute_unique(nums);

        ans.sort_unstable();
        res.sort_unstable();

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let nums = vec![1, 1, 2];
        let mut ans = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];

        let mut res = Solution::permute_unique(nums);

        ans.sort_unstable();
        res.sort_unstable();

        assert_eq!(res, ans);
    }
}
