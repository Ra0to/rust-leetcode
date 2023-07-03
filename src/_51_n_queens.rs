pub struct Solution;

fn place_queens(
    queens_left: usize,
    n: usize,
    v_lines: &mut Vec<bool>,
    d_first: &mut Vec<bool>,
    d_second: &mut Vec<bool>,
    board: &mut Vec<Vec<bool>>,
    ans: &mut Vec<Vec<String>>,
) {
    if queens_left == 0 {
        let ans_vec = board
            .iter()
            .map(|line| {
                line.iter()
                    .map(|&place| if place { 'Q' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<String>>();

        for solution in ans.iter() {
            if *solution == ans_vec {
                return;
            }
        }

        ans.push(ans_vec);
        return;
    }

    let y = queens_left - 1;
    for x in 0..n {
        if v_lines[x] {
            continue;
        }

        let df = x + y;
        let ds = n + y - x - 1;

        if d_first[df] {
            continue;
        }

        if d_second[ds] {
            continue;
        }

        v_lines[x] = true;
        d_first[df] = true;
        d_second[ds] = true;
        board[y][x] = true;

        place_queens(queens_left - 1, n, v_lines, d_first, d_second, board, ans);

        v_lines[x] = false;
        d_first[df] = false;
        d_second[ds] = false;
        board[y][x] = false;
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut v_lines = vec![false; n];
        // (x + y) diagonales from bottom left to top right. Start indexing from top left
        let mut d_first = vec![false; 2 * n - 1];
        // (y -x + n - 1) diagonales from bottom right to top left. Start indexing from bottom left
        let mut d_second = vec![false; 2 * n - 1];

        let mut board = vec![vec![false; n]; n];

        let mut ans: Vec<Vec<String>> = Vec::new();

        place_queens(
            n,
            n,
            &mut v_lines,
            &mut d_first,
            &mut d_second,
            &mut board,
            &mut ans,
        );

        ans
    }
}

#[cfg(test)]
mod _51_tests {
    use crate::_51_n_queens::*;

    #[test]
    fn test1() {
        let n = 1;
        let mut ans = vec![vec![String::from("Q")]];

        let mut res = Solution::solve_n_queens(n);

        res.sort();
        ans.sort();
        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let n = 4;
        let mut ans = vec![
            vec![
                String::from(".Q.."),
                String::from("...Q"),
                String::from("Q..."),
                String::from("..Q."),
            ],
            vec![
                String::from("..Q."),
                String::from("Q..."),
                String::from("...Q"),
                String::from(".Q.."),
            ],
        ];

        let mut res = Solution::solve_n_queens(n);

        res.sort();
        ans.sort();
        assert_eq!(res, ans);
    }
}
