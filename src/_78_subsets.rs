pub struct Solution;

fn fill(len: usize, ans: &mut Vec<Vec<i32>>, nums: &Vec<i32>) {
    let mut list = vec![0; len as usize];
    let mut ind = 0 as usize;
    loop {
        list[ind] += 1;

        if list[ind] > nums.len() as i32 {
            if ind == 0 {
                break;
            }

            ind -= 1;
        } else if ind == len - 1 {
            ans.push(list.iter().map(|&x| nums[(x - 1) as usize]).collect());
        } else {
            list[ind + 1] = list[ind];
            ind += 1;
        }
    }
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::<Vec<i32>>::new();

        ans.push(vec![]);

        for len in 1..=nums.len() {
            fill(len, &mut ans, &nums)
        }

        ans
    }
}

#[cfg(test)]
mod _78_tests {
    use crate::_78_subsets::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3];
        let mut ans = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];

        let mut res = Solution::subsets(nums);

        res.iter_mut().for_each(|list| list.sort());
        ans.iter_mut().for_each(|list| list.sort());
        res.sort();
        ans.sort();

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let nums = vec![0];
        let mut ans = vec![vec![], vec![0]];

        let mut res = Solution::subsets(nums);

        res.iter_mut().for_each(|list| list.sort());
        ans.iter_mut().for_each(|list| list.sort());
        res.sort();
        ans.sort();

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let nums = vec![3, 2, 1];
        let mut ans = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];

        let mut res = Solution::subsets(nums);

        res.iter_mut().for_each(|list| list.sort());
        ans.iter_mut().for_each(|list| list.sort());
        res.sort();
        ans.sort();

        assert_eq!(res, ans);
    }
}
