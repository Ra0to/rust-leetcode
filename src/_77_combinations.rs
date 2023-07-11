pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut list = vec![0; k as usize];
        let mut ans = Vec::<Vec<i32>>::new();

        let mut ind = 0 as usize;
        loop {
            list[ind] += 1;

            if list[ind] > n {
                if ind == 0 {
                    break;
                }

                ind -= 1;
            } else if ind == (k - 1) as usize {
                ans.push(list.clone());
            } else {
                list[ind + 1] = list[ind];
                ind += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod _77_tests {
    use crate::{_77_combinations::*, helpers::ConvertableToVecMatrix};

    #[test]
    fn test1() {
        let n = 4;
        let k = 2;
        let mut ans = [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]].to_vecs();

        let mut res = Solution::combine(n, k);

        res.sort();
        ans.sort();

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let n = 1;
        let k = 1;
        let mut ans = [[1]].to_vecs();

        let mut res = Solution::combine(n, k);

        res.sort();
        ans.sort();

        assert_eq!(res, ans);
    }
}
