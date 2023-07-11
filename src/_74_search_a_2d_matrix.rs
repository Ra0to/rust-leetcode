use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let find_line_res = matrix.binary_search_by(|line| {
            if line[0] > target {
                Ordering::Greater
            } else {
                if line[line.len() - 1] < target {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        });

        match find_line_res {
            Ok(line_index) => matrix[line_index].binary_search(&target).is_ok(),
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod _74_tests {
    use crate::_74_search_a_2d_matrix::*;
    use crate::helpers::test_helpers::ConvertableToVecMatrix;

    #[test]
    fn test1() {
        let matrix = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vecs();
        let target = 3;
        let ans = true;

        let res = Solution::search_matrix(matrix, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let matrix = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vecs();
        let target = 0;
        let ans = false;

        let res = Solution::search_matrix(matrix, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let matrix = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vecs();
        let target = 1000;
        let ans = false;

        let res = Solution::search_matrix(matrix, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let matrix = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vecs();
        let target = 1;
        let ans = true;

        let res = Solution::search_matrix(matrix, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let matrix = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vecs();
        let target = 7;
        let ans = true;

        let res = Solution::search_matrix(matrix, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test6() {
        let matrix = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vecs();
        let target = 10;
        let ans = true;

        let res = Solution::search_matrix(matrix, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test7() {
        let matrix = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vecs();
        let target = 11;
        let ans = true;

        let res = Solution::search_matrix(matrix, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test8() {
        let matrix = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vecs();
        let target = 12;
        let ans = false;

        let res = Solution::search_matrix(matrix, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test9() {
        let matrix = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vecs();
        let target = 20;
        let ans = true;

        let res = Solution::search_matrix(matrix, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test10() {
        let matrix = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vecs();
        let target = 23;
        let ans = true;

        let res = Solution::search_matrix(matrix, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test11() {
        let matrix = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vecs();
        let target = 60;
        let ans = true;

        let res = Solution::search_matrix(matrix, target);

        assert_eq!(res, ans);
    }

    #[test]
    fn test12() {
        let matrix = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vecs();
        let target = 21;
        let ans = false;

        let res = Solution::search_matrix(matrix, target);

        assert_eq!(res, ans);
    }
}
