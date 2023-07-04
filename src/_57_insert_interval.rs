struct Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            intervals.push(new_interval);
            return intervals;
        }

        if intervals[intervals.len() - 1][1] < new_interval[0] {
            intervals.push(new_interval);
            return intervals;
        }

        if intervals[0][0] > new_interval[1] {
            intervals.insert(0, new_interval);
            return intervals;
        }

        let begin = intervals.partition_point(|interval| {
            !(new_interval[0] <= interval[0]
                || (new_interval[0] > interval[0] && new_interval[0] <= interval[1]))
        });

        let end = intervals.partition_point(|interval| {
            new_interval[1] >= interval[1]
                || (new_interval[1] >= interval[0] && new_interval[1] < interval[1])
        }) - 1;

        if begin > end {
            intervals.insert(begin, new_interval);
            return intervals;
        }

        intervals[begin][0] = new_interval[0].min(intervals[begin][0]);
        intervals[begin][1] = new_interval[1].max(intervals[end][1]);

        if end > begin {
            for index in (begin + 1..=end).rev() {
                intervals.remove(index);
            }
        }

        intervals
    }
}

#[cfg(test)]
mod _57_tests {
    use crate::_57_insert_interval::*;

    #[test]
    fn test1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let ans = vec![vec![1, 5], vec![6, 9]];

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        let ans = vec![vec![1, 2], vec![3, 10], vec![12, 16]];

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let intervals = vec![vec![1, 5], vec![8, 10], vec![12, 15]];
        let new_interval = vec![0, 50];
        let ans = vec![vec![0, 50]];

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let intervals = vec![vec![1, 5], vec![8, 10], vec![12, 15]];
        let new_interval = vec![8, 10];
        let ans = vec![vec![1, 5], vec![8, 10], vec![12, 15]];

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let intervals = vec![];
        let new_interval = vec![8, 10];
        let ans = vec![vec![8, 10]];

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test6() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![6, 8];
        let ans = vec![vec![1, 5], vec![6, 8]];

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test7() {
        let intervals = vec![vec![8, 10]];
        let new_interval = vec![1, 2];
        let ans = vec![vec![1, 2], vec![8, 10]];

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test8() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![2, 3];
        let ans = vec![vec![1, 5]];

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test9() {
        let intervals = vec![vec![3, 5], vec![12, 15]];
        let new_interval = vec![6, 6];
        let ans = vec![vec![3, 5], vec![6, 6], vec![12, 15]];

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }
}
