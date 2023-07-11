pub struct Solution;

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
    use crate::{_57_insert_interval::*, helpers::ConvertableToVecMatrix};

    #[test]
    fn test1() {
        let intervals = [[1, 3], [6, 9]].to_vecs();
        let new_interval = vec![2, 5];
        let ans = [[1, 5], [6, 9]].to_vecs();

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let intervals = [[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]].to_vecs();
        let new_interval = vec![4, 8];
        let ans = [[1, 2], [3, 10], [12, 16]].to_vecs();

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let intervals = [[1, 5], [8, 10], [12, 15]].to_vecs();
        let new_interval = vec![0, 50];
        let ans = [[0, 50]].to_vecs();

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let intervals = [[1, 5], [8, 10], [12, 15]].to_vecs();
        let new_interval = vec![8, 10];
        let ans = [[1, 5], [8, 10], [12, 15]].to_vecs();

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let intervals = vec![];
        let new_interval = vec![8, 10];
        let ans = [[8, 10]].to_vecs();

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test6() {
        let intervals = [[1, 5]].to_vecs();
        let new_interval = vec![6, 8];
        let ans = [[1, 5], [6, 8]].to_vecs();

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test7() {
        let intervals = [[8, 10]].to_vecs();
        let new_interval = vec![1, 2];
        let ans = [[1, 2], [8, 10]].to_vecs();

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test8() {
        let intervals = [[1, 5]].to_vecs();
        let new_interval = vec![2, 3];
        let ans = [[1, 5]].to_vecs();

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }

    #[test]
    fn test9() {
        let intervals = [[3, 5], [12, 15]].to_vecs();
        let new_interval = vec![6, 6];
        let ans = [[3, 5], [6, 6], [12, 15]].to_vecs();

        let res = Solution::insert(intervals, new_interval);

        assert_eq!(res, ans);
    }
}
