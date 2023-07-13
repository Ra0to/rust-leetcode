pub struct Solution;

fn check(
    x: usize,
    y: usize,
    letter_index: usize,
    board: &mut Vec<Vec<char>>,
    word: &Vec<char>,
) -> bool {
    if letter_index >= word.len() {
        return true;
    }

    if y >= board.len() || x >= board[0].len() {
        return false;
    }

    if board[y][x] != word[letter_index] {
        return false;
    }

    board[y][x] = ' ';

    let res = (x > 0 && check(x - 1, y, letter_index + 1, board, word))
        || check(x + 1, y, letter_index + 1, board, word)
        || (y > 0 && check(x, y - 1, letter_index + 1, board, word))
        || check(x, y + 1, letter_index + 1, board, word);

    board[y][x] = word[letter_index];

    res
}

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<char>>();

        for y in 0..board.len() {
            for x in 0..board[0].len() {
                if check(x, y, 0, &mut board, &word) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod _79_tests {
    use crate::{_79_word_search::*, helpers::ConvertableToVecMatrix};

    #[test]
    fn test1() {
        let board = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ]
        .to_vecs();
        let word = String::from("ABCCED");
        let ans = true;

        let res = Solution::exist(board, word);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let board = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ]
        .to_vecs();
        let word = String::from("SEE");
        let ans = true;

        let res = Solution::exist(board, word);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let board = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ]
        .to_vecs();
        let word = String::from("ABCB");
        let ans = false;

        let res = Solution::exist(board, word);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let board = [['A']].to_vecs();
        let word = String::from("ABCB");
        let ans = false;

        let res = Solution::exist(board, word);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        // Testcase #48
        let board = [['a', 'b']].to_vecs();
        let word = String::from("ba");
        let ans = true;

        let res = Solution::exist(board, word);

        assert_eq!(res, ans);
    }

    #[test]
    fn test6() {
        // Testcase #65
        let board = [['C', 'A', 'A'], ['A', 'A', 'A'], ['B', 'C', 'D']].to_vecs();
        let word = String::from("AAB");
        let ans = true;

        let res = Solution::exist(board, word);

        assert_eq!(res, ans);
    }

    #[test]
    fn test7() {
        // Testcase #78
        let board = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'E', 'S'],
            ['A', 'D', 'E', 'E'],
        ]
        .to_vecs();
        let word = String::from("ABCESEEEFS");
        let ans = true;

        let res = Solution::exist(board, word);

        assert_eq!(res, ans);
    }
}
