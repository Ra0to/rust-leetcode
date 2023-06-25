use std::ops::Deref;

struct Solution;

fn is_char(oc: Option<&char>, ch: char) -> bool {
    match oc {
        Some(c) => *c == ch,
        None => false,
    }
}

fn is_digit(oc: Option<&char>) -> bool {
    match oc {
        Some(c) => *c >= '0' && *c <= '9',
        None => false,
    }
}
impl Solution {
    // TODO refactor this. Very complex solution. Unsupportable and bad metrics.
    pub fn is_number(s: String) -> bool {
        // ^[+-]?((\d+\.?)|(\d+\.\d+)|(\.\d+))([eE][+-]?\d+)?$

        // ^[+-]?(@|@.|@.@|.@)([eE][+-]?@)?$
        let chars = s.chars().collect::<Vec<char>>();
        let mut has_e = false;
        let mut has_nums_before_e = false;
        let mut has_nums_after_e = false;
        let mut has_period = false;
        let mut has_sign = false;
        dbg!(s);

        for (ind, c) in chars.iter().enumerate() {
            let p = if ind > 0 { chars.get(ind - 1) } else { None };
            let c = Some(c);
            let n = chars.get(ind + 1);
            dbg!(p);
            dbg!(c);
            dbg!(n);

            // Check invalid characters
            if !(is_digit(c)
                || is_char(c, 'e')
                || is_char(c, 'E')
                || is_char(c, '+')
                || is_char(c, '-')
                || is_char(c, '.'))
            {
                dbg!("Invalid char");
                return false;
            }

            if ind == 0 && (is_char(c, '+') || is_char(c, '-')) {
                continue;
            }

            if is_digit(c) {
                if has_e {
                    has_nums_after_e = true;
                } else {
                    has_nums_before_e = true;
                }

                continue;
            }

            if is_char(c, '.') {
                if has_e {
                    dbg!("No fractial numbers in exponent allowed");
                    return false;
                }
                if has_period {
                    dbg!("Already has period");
                    return false;
                }

                if !is_digit(p) && !is_digit(n) {
                    dbg!("No digits around period");
                    return false;
                }

                has_period = true;
                continue;
            }

            if is_char(c, 'e') || is_char(c, 'E') {
                if has_e {
                    dbg!("Already has e");
                    return false;
                }

                has_e = true;
                continue;
            }

            if is_char(c, '+') || is_char(c, '-') {
                if !has_e {
                    dbg!("Sign in wrong place");
                    return false;
                }

                if has_sign {
                    dbg!("Already has sign");

                    return false;
                }

                if !is_char(p, 'e') && !is_char(p, 'E') {
                    dbg!("Sign in wrong place in exponent");
                    return false;
                }

                if !is_digit(n) {
                    dbg!("Doesn't contains digit after sign");
                    return false;
                }

                has_sign = true;
                continue;
            }
        }

        return has_nums_before_e && (!has_e || (has_e && has_nums_after_e));
    }
}

#[cfg(test)]
mod _56_tests {
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
