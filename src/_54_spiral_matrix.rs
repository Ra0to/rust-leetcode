struct Solution;

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans = vec![0; m * n];
        let mut index = 0;
        let mut x = 0 as usize;
        let mut y = 0 as usize;

        let mut top = 0;
        let mut bottom = m - 1;
        let mut left = 0;
        let mut right = n - 1;

        let mut direction = Direction::Right;

        while index < ans.len() {
            ans[index] = matrix[y][x];
            index += 1;

            match direction {
                Direction::Right if x == right => {
                    direction = Direction::Down;
                    top += 1;
                }
                Direction::Down if y == bottom => {
                    direction = Direction::Left;
                    right -= 1;
                }
                Direction::Left if x == left => {
                    direction = Direction::Up;
                    bottom -= 1;
                }
                Direction::Up if y == top => {
                    direction = Direction::Right;
                    left += 1;
                }
                _ => (),
            }

            match direction {
                Direction::Right => x += 1,
                Direction::Down => y += 1,
                Direction::Left => x -= 1,
                Direction::Up => y -= 1,
            }
        }

        ans
    }
}

#[cfg(test)]
mod _54_tests {
    use crate::{_54_spiral_matrix::*, helpers::ConvertableToVecMatrix};

    #[test]
    fn test1() {
        let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].to_vecs();
        let ans = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];

        let res = Solution::spiral_order(matrix);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let matrix = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]].to_vecs();
        let ans = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];

        let res = Solution::spiral_order(matrix);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let matrix = [[1]].to_vecs();
        let ans = vec![1];

        let res = Solution::spiral_order(matrix);

        assert_eq!(res, ans);
    }
}
