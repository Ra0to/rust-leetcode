pub struct Solution;

const MAX_N: usize = 9;

fn get_bit(storage: &i32, bit: usize) -> bool {
    storage & (1 << bit) > 0
}

fn set_bit(storage: &mut i32, bit: usize) {
    *storage = *storage | (1 << bit)
}

fn unset_bit(storage: &mut i32, bit: usize) {
    *storage = *storage & (i32::MAX - (1 << bit))
}

fn place_queens(
    queens_left: usize,
    n: usize,
    v_lines: &mut i32,
    d_first: &mut i32,
    d_second: &mut i32,
    board: &mut [i32; MAX_N],
    ans: &mut Vec<Vec<String>>,
) {
    if queens_left == 0 {
        let ans_vec = board
            .iter()
            .take(n)
            .map(|line| {
                (0..n)
                    .map(|pos| if get_bit(line, pos) { 'Q' } else { '.' })
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
        if get_bit(v_lines, x) {
            continue;
        }

        let df = x + y;
        let ds = n + y - x - 1;

        if get_bit(d_first, df) {
            continue;
        }

        if get_bit(d_second, ds) {
            continue;
        }

        set_bit(v_lines, x);
        set_bit(d_first, df);
        set_bit(d_second, ds);
        set_bit(&mut board[y], x);

        place_queens(queens_left - 1, n, v_lines, d_first, d_second, board, ans);

        unset_bit(v_lines, x);
        unset_bit(d_first, df);
        unset_bit(d_second, ds);
        unset_bit(&mut board[y], x);
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;

        let mut v_lines: i32 = 0;
        // (x + y) diagonales from bottom left to top right. Start indexing from top left
        let mut d_first: i32 = 0;
        // (y -x + n - 1) diagonales from bottom right to top left. Start indexing from bottom left
        let mut d_second: i32 = 0;

        let mut board = [0 as i32; MAX_N];

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
