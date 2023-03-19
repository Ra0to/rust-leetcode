struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        for iter in 0..(len / 2) {
            let offset = (iter, iter);
            let size = len - 2 * iter;

            for cell_index in 0..(size - 1) {
                let (top, right, bottom, left) = Self::get_points(cell_index, offset, size);

                let tmp = matrix[top.0][top.1];
                matrix[top.0][top.1] = matrix[left.0][left.1];
                matrix[left.0][left.1] = matrix[bottom.0][bottom.1];
                matrix[bottom.0][bottom.1] = matrix[right.0][right.1];
                matrix[right.0][right.1] = tmp;
            }
        }
    }

    fn get_points(
        index: usize,
        offset: (usize, usize),
        size: usize,
    ) -> (
        (usize, usize),
        (usize, usize),
        (usize, usize),
        (usize, usize),
    ) {
        (
            (offset.0 + 0, offset.1 + index),
            (offset.0 + index, offset.1 + size - 1),
            (offset.0 + size - 1, offset.1 + size - 1 - index),
            (offset.0 + size - 1 - index, offset.1 + 0),
        )
    }
}

#[cfg(test)]
mod _48_tests {
    use crate::_48_rotate_image::*;

    #[test]
    fn test1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let ans = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];

        Solution::rotate(&mut matrix);

        assert_eq!(matrix, ans);
    }

    #[test]
    fn test2() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let ans = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];

        Solution::rotate(&mut matrix);

        assert_eq!(matrix, ans);
    }
}
