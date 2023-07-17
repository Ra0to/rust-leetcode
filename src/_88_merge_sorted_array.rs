pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }

        let n = n as usize;
        let m = m as usize;

        let mut ind1 = 0;
        let mut ind2 = 0;

        for ind in (0..nums1.len()).rev() {
            if ind2 >= n || ind1 < m && nums1[m - ind1 - 1] >= nums2[n - ind2 - 1] {
                nums1.swap(ind, m - ind1 - 1);
                ind1 += 1;
            } else {
                nums1[ind] = nums2[n - ind2 - 1];
                ind2 += 1;
            }
        }
    }
}

#[cfg(test)]
mod _88_tests {
    use crate::_88_merge_sorted_array::*;

    #[test]
    fn test1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        let ans = vec![1, 2, 2, 3, 5, 6];

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, ans);
    }

    #[test]
    fn test2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        let ans = vec![1];

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, ans);
    }

    #[test]
    fn test3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        let ans = vec![1];

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, ans);
    }

    #[test]
    fn test4() {
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![1, 2, 3];
        let n = 3;
        let ans = vec![1, 2, 3, 4, 5, 6];

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, ans);
    }
}
