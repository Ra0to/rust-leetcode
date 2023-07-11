pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // 0 - red, 1 - white, 2 - blue
        let mut colors = [0 as usize; 3];
        nums.iter().for_each(|&color| colors[color as usize] += 1);

        let mut color_index = 0 as usize;
        for i in 0..nums.len() {
            while colors[color_index] == 0 {
                color_index += 1;
            }
            nums[i] = color_index as i32;
            colors[color_index] -= 1;
        }
    }
}

#[cfg(test)]
mod _75_tests {
    use crate::_75_sort_colors::*;

    #[test]
    fn test1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let ans = vec![0, 0, 1, 1, 2, 2];

        Solution::sort_colors(&mut nums);

        assert_eq!(nums, ans);
    }

    #[test]
    fn test2() {
        let mut nums = vec![2, 0, 1];
        let ans = vec![0, 1, 2];

        Solution::sort_colors(&mut nums);

        assert_eq!(nums, ans);
    }

    #[test]
    fn test3() {
        let mut nums = vec![0];
        let ans = vec![0];

        Solution::sort_colors(&mut nums);

        assert_eq!(nums, ans);
    }

    #[test]
    fn test4() {
        let mut nums = vec![1, 1, 1, 1, 1];
        let ans = vec![1, 1, 1, 1, 1];

        Solution::sort_colors(&mut nums);

        assert_eq!(nums, ans);
    }
}
