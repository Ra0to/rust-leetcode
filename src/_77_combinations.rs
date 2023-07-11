pub struct Solution;

fn fill(
    n: usize,
    k: usize,
    i: usize,
    min: usize,
    list: &mut Vec<i32>,
    used: &mut Vec<bool>,
    ans: &mut Vec<Vec<i32>>,
) {
    if i >= k {
        ans.push(list.clone());
        return;
    }

    for num in min..=n {
        if used[num] {
            continue;
        }

        used[num] = true;
        list[i] = num as i32;

        fill(n, k, i + 1, num + 1, list, used, ans);

        used[num] = false;
    }
}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let k = k as usize;
        let mut used = vec![false; n + 1];
        let mut list = vec![0; k];
        let mut ans = Vec::<Vec<i32>>::new();

        fill(n, k, 0, 1, &mut list, &mut used, &mut ans);

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
