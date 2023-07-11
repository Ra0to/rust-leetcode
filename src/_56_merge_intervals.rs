pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|x| x[0]);
        let mut ans = Vec::<Vec<i32>>::new();

        let mut start = intervals[0][0];
        let mut end = intervals[0][1];

        for i in 1..intervals.len() {
            let interval = &intervals[i];
            let fir = interval[0];
            let sec = interval[1];

            if fir <= end {
                end = end.max(sec);
                continue;
            }

            ans.push(vec![start, end]);
            start = fir;
            end = sec;
        }
        ans.push(vec![start, end]);

        ans
    }
}

#[cfg(test)]
mod _56_tests {
    use crate::{_56_merge_intervals::*, helpers::ConvertableToVecMatrix};

    #[test]
    fn test1() {
        let intervals = [[1, 3], [2, 6], [8, 10], [15, 18]].to_vecs();
        let ans = [[1, 6], [8, 10], [15, 18]].to_vecs();

        let res = Solution::merge(intervals);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let intervals = [[1, 4], [4, 5]].to_vecs();
        let ans = [[1, 5]].to_vecs();

        let res = Solution::merge(intervals);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let intervals = [[15, 18], [8, 10], [1, 3], [2, 6], [8, 10], [15, 18]].to_vecs();
        let ans = [[1, 6], [8, 10], [15, 18]].to_vecs();

        let res = Solution::merge(intervals);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let intervals = [[1, 1], [1, 1], [1, 1], [1, 1], [1, 1], [1, 1]].to_vecs();
        let ans = [[1, 1]].to_vecs();

        let res = Solution::merge(intervals);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let intervals = [[1, 4], [2, 3]].to_vecs();
        let ans = [[1, 4]].to_vecs();

        let res = Solution::merge(intervals);

        assert_eq!(res, ans);
    }
}
