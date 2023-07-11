pub struct Solution;

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0 as i32; n]; n];
        let total = n * n;

        let mut index = 0;
        let mut x = 0 as usize;
        let mut y = 0 as usize;

        let mut top = 0;
        let mut bottom = n - 1;
        let mut left = 0;
        let mut right = n - 1;

        let mut direction = Direction::Right;

        while index < total {
            index += 1;
            matrix[y][x] = index as i32;

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

        matrix
    }
}

#[cfg(test)]
mod _59_tests {
    use crate::{_59_spiral_matrix_ii::*, helpers::ConvertableToVecMatrix};

    #[test]
    fn test1() {
        let n = 3;
        let ans = [[1, 2, 3], [8, 9, 4], [7, 6, 5]].to_vecs();

        let res = Solution::generate_matrix(n);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let n = 1;
        let ans = [[1]].to_vecs();

        let res = Solution::generate_matrix(n);

        assert_eq!(res, ans);
    }
}
