pub struct Solution;

impl Solution {
    const ASTERIKS: u8 = '*' as u8;
    const QUESTION_MARK: u8 = '?' as u8;

    pub fn is_match(s: String, p: String) -> bool {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();

        let mut s_ind = 0;
        let mut p_ind = 0;

        let mut asteriks_pos: Option<usize> = None;
        let mut string_pos_at_asteriks: usize = 0;

        while s_ind < s_bytes.len() {
            if p_ind < p_bytes.len()
                && (s_bytes[s_ind] == p_bytes[p_ind] || p_bytes[p_ind] == Self::QUESTION_MARK)
            {
                s_ind += 1;
                p_ind += 1;
                continue;
            }

            if p_ind < p_bytes.len() && p_bytes[p_ind] == Self::ASTERIKS {
                asteriks_pos = Some(p_ind);
                string_pos_at_asteriks = s_ind;
                p_ind += 1;
                continue;
            }

            if let Some(asteriks_pos) = asteriks_pos {
                p_ind = asteriks_pos + 1;
                string_pos_at_asteriks += 1;
                s_ind = string_pos_at_asteriks;
                continue;
            }

            return false;
        }

        while p_ind < p_bytes.len() && p_bytes[p_ind] == Self::ASTERIKS {
            p_ind += 1;
        }

        return p_ind >= p_bytes.len();
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
