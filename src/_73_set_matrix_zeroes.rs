pub struct Solution;

fn find_zero(mut matrix: &mut Vec<Vec<i32>>, from_x: usize, from_y: usize) {
    for y in from_y..matrix.len() {
        let x_start = if y == from_y { from_x } else { 0 };
        for x in x_start..matrix[0].len() {
            if matrix[y][x] != 0 {
                continue;
            }

            if x == matrix[0].len() - 1 {
                find_zero(&mut matrix, 0, y + 1);
            } else {
                find_zero(&mut matrix, x + 1, y);
            }

            for i in 0..matrix.len() {
                matrix[i][x] = 0;
            }

            for i in 0..matrix[0].len() {
                matrix[y][i] = 0;
            }

            return;
        }
    }
}

fn set_zeroes_recursion(matrix: &mut Vec<Vec<i32>>) {
    find_zero(matrix, 0, 0);
}

fn set_zeroes_first_line(matrix: &mut Vec<Vec<i32>>) {
    let has_zero_in_first_col = matrix.iter().any(|line| line[0] == 0);
    let has_zero_in_first_row = matrix[0].iter().any(|&elem| elem == 0);

    for y in 1..matrix.len() {
        for x in 1..matrix[0].len() {
            if matrix[y][x] == 0 {
                matrix[y][0] = 0;
                matrix[0][x] = 0;
            }
        }
    }

    for y in 1..matrix.len() {
        for x in 1..matrix[0].len() {
            if matrix[0][x] == 0 || matrix[y][0] == 0 {
                matrix[y][x] = 0;
            }
        }
    }

    if has_zero_in_first_col {
        matrix.iter_mut().for_each(|elem| elem[0] = 0);
    }

    if has_zero_in_first_row {
        matrix[0].iter_mut().for_each(|elem| *elem = 0);
    }
}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        // Original solution
        // set_zeroes_recursion(matrix);
        // Alternative solution
        set_zeroes_first_line(matrix);
    }

    pub fn original_set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        set_zeroes_recursion(matrix);
    }
}

#[cfg(test)]
mod _73_tests {
    use crate::_73_set_matrix_zeroes::*;
    use crate::helpers::test_helpers::ConvertableToVecMatrix;

    #[test]
    fn test1() {
        let mut matrix = [[1, 1, 1], [1, 0, 1], [1, 1, 1]].to_vecs();
        let ans = [[1, 0, 1], [0, 0, 0], [1, 0, 1]].to_vecs();

        Solution::set_zeroes(&mut matrix);

        assert_eq!(matrix, ans);
    }

    #[test]
    fn test2() {
        let mut matrix = [[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]].to_vecs();
        let ans = [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]].to_vecs();

        Solution::set_zeroes(&mut matrix);

        assert_eq!(matrix, ans);
    }

    #[test]
    fn test3() {
        const MAX_COUNT: usize = 200;
        // Default stack size on my machine not enough, leetcode use more stack size (may be only for this problem)
        const STACK_SIZE: usize = 16 * 1024 * 1024;

        // From: https://www.reddit.com/r/rust/comments/872fc4/comment/dwa72rq/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
        // Spawn thread with explicit stack size
        let child = std::thread::Builder::new()
            .stack_size(STACK_SIZE)
            .spawn(move || {
                let mut matrix = vec![vec![0; MAX_COUNT]; MAX_COUNT];
                let ans = vec![vec![0; MAX_COUNT]; MAX_COUNT];
                Solution::set_zeroes(&mut matrix);
                assert_eq!(matrix, ans);
            })
            .unwrap();

        // Wait for thread to join
        child.join().unwrap();
    }

    #[test]
    fn test4() {
        let mut matrix = [[1, 2, 3, 4], [5, 0, 7, 8], [0, 10, 11, 12], [13, 14, 15, 0]].to_vecs();
        let ans = [[0, 0, 3, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]].to_vecs();

        Solution::set_zeroes(&mut matrix);

        assert_eq!(matrix, ans);
    }

    #[test]
    fn test5() {
        let mut matrix = [
            [0, 0, 0, 5],
            [4, 3, 1, 4],
            [0, 1, 1, 4],
            [1, 2, 1, 3],
            [0, 0, 1, 1],
        ]
        .to_vecs();
        let ans = [
            [0, 0, 0, 0],
            [0, 0, 0, 4],
            [0, 0, 0, 0],
            [0, 0, 0, 3],
            [0, 0, 0, 0],
        ]
        .to_vecs();

        Solution::set_zeroes(&mut matrix);

        assert_eq!(matrix, ans);
    }
}
