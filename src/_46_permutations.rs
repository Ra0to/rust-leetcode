struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::<Vec<i32>>::new();
        // nums.len <= 6
        let is_used: i8 = 0b0000_0000;
        let mut current = vec![0; nums.len()];

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

        for (i, num) in nums.iter().enumerate() {
            let flag = 1 << i;
            if is_used & flag != 0 {
                continue;
            }

            current[index] = *num;
            Self::fill_answers(&nums, is_used | flag, &mut current, index + 1, &mut ans);
        }
    }
}

#[cfg(test)]
mod _46_tests {
    use crate::{_46_permutations::*, helpers::ConvertableToVecMatrix};

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3];
        let mut ans = [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ]
        .to_vecs();

        let mut res = Solution::permute(nums);

        ans.sort_unstable();
        res.sort_unstable();

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 1];
        let mut ans = [[0, 1], [1, 0]].to_vecs();

        let mut res = Solution::permute(nums);

        ans.sort_unstable();
        res.sort_unstable();

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let nums = vec![1];
        let mut ans = [[1]].to_vecs();

        let mut res = Solution::permute(nums);

        ans.sort_unstable();
        res.sort_unstable();

        assert_eq!(res, ans);
    }
}
