pub struct Solution;

const RED: i32 = 0;
const BLUE: i32 = 2;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut red_index = 0 as usize;
        let mut blue_index = nums.len() - 1;

        let mut index = 0 as usize;

        while index <= blue_index {
            match nums[index] {
                RED if red_index != index => {
                    nums.swap(index, red_index);
                    red_index += 1;
                }
                RED => {
                    red_index += 1;
                    index += 1;
                }
                BLUE => {
                    nums.swap(index, blue_index);

                    if blue_index == 0 {
                        break;
                    }

                    blue_index -= 1;
                }
                _ => index += 1,
            }
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

    #[test]
    fn test5() {
        let mut nums = vec![1];
        let ans = vec![1];

        Solution::sort_colors(&mut nums);

        assert_eq!(nums, ans);
    }

    #[test]
    fn test6() {
        let mut nums = vec![2];
        let ans = vec![2];

        Solution::sort_colors(&mut nums);

        assert_eq!(nums, ans);
    }
}
