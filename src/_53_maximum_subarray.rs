struct Solution;

impl Solution {
    // Kadane's Algorithm
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut best_sum = i32::MIN;
        let mut current_sum = 0;

        for &x in nums.iter() {
            current_sum = x.max(current_sum + x);
            best_sum = best_sum.max(current_sum);
        }

        best_sum
    }
}

#[cfg(test)]
mod _53_tests {
    use crate::_53_maximum_subarray::*;

    #[test]
    fn test1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let ans = 6;

        let res = Solution::max_sub_array(nums);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let nums = vec![1];
        let ans = 1;

        let res = Solution::max_sub_array(nums);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let nums = vec![5, 4, -1, 7, 8];
        let ans = 23;

        let res = Solution::max_sub_array(nums);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let mut nums = vec![-1; 4 as usize];
        nums.push(0);
        let ans = 0;

        let res = Solution::max_sub_array(nums);

        assert_eq!(res, ans);
    }
}
