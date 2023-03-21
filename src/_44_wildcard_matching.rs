use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut checked = HashMap::<(usize, usize), bool>::new();
        Self::is_matching(&s, &p, 0, 0, &mut checked)
    }

    fn is_matching(
        s: &str,
        p: &str,
        s_ind: usize,
        p_ind: usize,
        checked: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        if checked.contains_key(&(s_ind, p_ind)) {
            return checked[&(s_ind, p_ind)];
        }

        if s_ind >= s.len() && p_ind >= p.len() {
            checked.insert((s_ind, p_ind), true);
            return true;
        }

        if p_ind >= p.len() {
            checked.insert((s_ind, p_ind), false);
            return false;
        }

        if s_ind >= s.len() {
            let res = p.chars().skip(p_ind).all(|c| c == '*');
            checked.insert((s_ind, p_ind), res);
            return res;
        }

        let p_chr = p.chars().nth(p_ind).unwrap();
        let s_chr = s.chars().nth(s_ind).unwrap();

        if p_chr == '?' {
            let res = Self::is_matching(s, p, s_ind + 1, p_ind + 1, checked);
            checked.insert((s_ind, p_ind), res);
            return res;
        }

        if p_chr == '*' {
            // We can replace "**" (any amount of asteriks) with "*", because they behave in same way
            if p_ind > 0 && p.chars().nth(p_ind - 1).unwrap() == '*' {
                return Self::is_matching(s, p, s_ind, p_ind + 1, checked);
            }

            let res =
                // Match 0 symbols
                Self::is_matching(s, p, s_ind, p_ind + 1, checked) ||
                // Match symbol and finish asteriks        
                Self::is_matching(s, p, s_ind + 1, p_ind + 1, checked) ||
                // Match symbol and continue using asteriks
                Self::is_matching(s, p, s_ind + 1, p_ind, checked);

            checked.insert((s_ind, p_ind), res);
            return res;
        }

        if p_chr == s_chr {
            let res = Self::is_matching(s, p, s_ind + 1, p_ind + 1, checked);
            checked.insert((s_ind, p_ind), res);
            return res;
        }

        checked.insert((s_ind, p_ind), false);
        false
    }
}

#[cfg(test)]
mod _44_tests {
    use crate::_44_wildcard_matching::*;

    #[test]
    fn test1() {
        let s = String::from("aa");
        let p = String::from("a");
        let ans = false;

        let res = Solution::is_match(s, p);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let s = String::from("aa");
        let p = String::from("*");
        let ans = true;

        let res = Solution::is_match(s, p);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let s = String::from("cb");
        let p = String::from("?a");
        let ans = false;

        let res = Solution::is_match(s, p);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let s = String::from("");
        let p = String::from("");
        let ans = true;

        let res = Solution::is_match(s, p);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let s = String::from("xxx");
        let p = String::from("???");
        let ans = true;

        let res = Solution::is_match(s, p);

        assert_eq!(res, ans);
    }

    #[test]
    fn test6() {
        let s = String::from("xxx");
        let p = String::from("?*?");
        let ans = true;

        let res = Solution::is_match(s, p);

        assert_eq!(res, ans);
    }

    #[test]
    fn test7() {
        let s = String::from("xxx");
        let p = String::from("??*x");
        let ans = true;

        let res = Solution::is_match(s, p);

        assert_eq!(res, ans);
    }

    #[test]
    fn test8() {
        let s = String::from("xx");
        let p = String::from("**");
        let ans = true;

        let res = Solution::is_match(s, p);

        assert_eq!(res, ans);
    }

    #[test]
    fn test9() {
        let s = String::from("");
        let p = String::from("**");
        let ans = true;

        let res = Solution::is_match(s, p);

        assert_eq!(res, ans);
    }

    #[test]
    fn test10() {
        let s = String::from("asdf");
        let p = String::from("");
        let ans = false;

        let res = Solution::is_match(s, p);

        assert_eq!(res, ans);
    }

    #[test]
    fn test11() {
        let s = String::from("asdf");
        let p = String::from("**asdf");
        let ans = true;

        let res = Solution::is_match(s, p);

        assert_eq!(res, ans);
    }

    #[test]
    fn test12() {
        let s = String::from("asdf");
        let p = String::from("asdf**");
        let ans = true;

        let res = Solution::is_match(s, p);

        assert_eq!(res, ans);
    }

    #[test]
    fn test13() {
        let s = String::from("babaaababaabababbbbbbaabaabbabababbaababbaaabbbaaab");
        let p = String::from("***bba**a*bbba**aab**");
        let ans = true;

        let res = Solution::is_match(s, p);

        assert_eq!(res, ans);
    }
}
