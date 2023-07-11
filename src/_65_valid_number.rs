pub struct Solution;

enum Match {
    Optional,
    One,
    OneAndMore,
    Any,
}

fn is_num(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_dot(c: char) -> bool {
    c == '.'
}

fn is_e(c: char) -> bool {
    c == 'e' || c == 'E'
}

fn is_sign(c: char) -> bool {
    c == '+' || c == '-'
}

fn try_match(s: &Vec<char>, i: &mut usize, predicate: fn(char) -> bool, match_type: Match) -> bool {
    if *i >= s.len() {
        return false;
    }

    let max_chars: usize = match match_type {
        Match::Optional => 1,
        Match::One => 1,
        Match::OneAndMore => usize::MAX,
        Match::Any => usize::MAX,
    };

    let mut cnt: usize = 0;

    while *i < s.len() && cnt < max_chars && predicate(s[*i]) {
        cnt += 1;
        *i += 1;
    }

    cnt > 0
}

fn try_match_fractional(s: &Vec<char>, mut i: &mut usize) -> bool {
    let mut res: bool;

    // ".01234" case
    res = try_match(s, &mut i, is_dot, Match::Optional);
    if res {
        return try_match(s, &mut i, is_num, Match::OneAndMore);
    }

    res = try_match(s, &mut i, is_num, Match::OneAndMore);

    if !res {
        return false;
    }

    try_match(s, &mut i, is_dot, Match::Optional);
    try_match(s, &mut i, is_num, Match::Any);

    return true;
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        // ^[+-]?((\d+\.?)|(\d+\.\d+)|(\.\d+))([eE][+-]?\d+)?$

        let chars = s.chars().collect::<Vec<char>>();

        let mut index: usize = 0;
        let mut res: bool;
        try_match(&chars, &mut index, is_sign, Match::Optional);
        res = try_match_fractional(&chars, &mut index);
        if !res {
            return false;
        }
        if index == s.len() {
            return true;
        }

        res = try_match(&chars, &mut index, is_e, Match::One);
        if !res {
            return false;
        }

        try_match(&chars, &mut index, is_sign, Match::Optional);
        res = try_match(&chars, &mut index, is_num, Match::OneAndMore);

        return res && index == s.len();
    }
}

#[cfg(test)]
mod _65_tests {
    use crate::_65_valid_number::*;

    #[test]
    fn test1() {
        let input = String::from("0");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let input = String::from("2");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let input = String::from("0089");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let input = String::from("-0.1");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let input = String::from("+3.14");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test6() {
        let input = String::from("4.");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test7() {
        let input = String::from("-.9");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test8() {
        let input = String::from("2e10");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test9() {
        let input = String::from("-90E3");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test10() {
        let input = String::from("3e+7");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test11() {
        let input = String::from("+6e-1");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test12() {
        let input = String::from("53.5e93");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test13() {
        let input = String::from("-123.456e789");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test14() {
        let input = String::from(".0");
        let ans = true;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test15() {
        let input = String::from("abc");
        let ans = false;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test16() {
        let input = String::from("1a");
        let ans = false;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test17() {
        let input = String::from("1e");
        let ans = false;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test18() {
        let input = String::from("e3");
        let ans = false;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test19() {
        let input = String::from("99e2.5");
        let ans = false;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test20() {
        let input = String::from("--5");
        let ans = false;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test21() {
        let input = String::from("-+3");
        let ans = false;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test22() {
        let input = String::from("95a54e53");
        let ans = false;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }

    #[test]
    fn test23() {
        let input = String::from("092e359-2");
        let ans = false;

        let res = Solution::is_number(input);

        assert_eq!(res, ans);
    }
}
