struct Solution;

fn fill_reachable(
    pos: usize,
    nums: &Vec<i32>,
    is_reachable: &mut Vec<bool>,
    is_visited: &mut Vec<bool>,
) {
    if is_visited[pos] {
        return;
    }

    is_visited[pos] = true;

    let max_pos = pos + nums[pos] as usize;

    if max_pos >= nums.len() - 1 {
        is_reachable[nums.len() - 1] = true;
        return;
    }

    for new_pos in (pos + 1..=max_pos).rev() {
        is_reachable[new_pos] = true;
        fill_reachable(new_pos, nums, is_reachable, is_visited);
    }
}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut is_reachable = vec![false; nums.len()];
        let mut is_visited = vec![false; nums.len()];
        is_reachable[0] = true;

        fill_reachable(0, &nums, &mut is_reachable, &mut is_visited);

        is_reachable[nums.len() - 1]
    }
}

#[cfg(test)]
mod _55_tests {
    use crate::_55_jump_game::*;

    #[test]
    fn test1() {
        let nums = vec![2, 3, 1, 1, 4];
        let ans = true;

        let res = Solution::can_jump(nums);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 2, 1, 0, 4];
        let ans = false;

        let res = Solution::can_jump(nums);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let len = 1e4 as usize;
        let mut nums = vec![0; len];
        for (index, el) in nums.iter_mut().enumerate().rev() {
            *el = (len as i32 - index as i32 - 2).max(0);
        }
        let ans = false;

        let res = Solution::can_jump(nums);

        assert_eq!(res, ans);
    }
}
