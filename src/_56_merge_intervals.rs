struct Solution;

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
    use crate::_56_merge_intervals::*;

    #[test]
    fn test1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let ans = vec![vec![1, 6], vec![8, 10], vec![15, 18]];

        let res = Solution::merge(intervals);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let ans = vec![vec![1, 5]];

        let res = Solution::merge(intervals);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let intervals = vec![
            vec![15, 18],
            vec![8, 10],
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15, 18],
        ];
        let ans = vec![vec![1, 6], vec![8, 10], vec![15, 18]];

        let res = Solution::merge(intervals);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let intervals = vec![
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
        ];
        let ans = vec![vec![1, 1]];

        let res = Solution::merge(intervals);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let intervals = vec![vec![1, 4], vec![2, 3]];
        let ans = vec![vec![1, 4]];

        let res = Solution::merge(intervals);

        assert_eq!(res, ans);
    }
}
