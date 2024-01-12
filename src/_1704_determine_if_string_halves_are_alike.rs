pub struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut vowels_diff = 0;

        let half_len = s.len() / 2;
        for (ind, c) in s.chars().enumerate() {
            vowels_diff += match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    if ind < half_len {
                        1
                    } else {
                        -1
                    }
                }
                _ => 0,
            };
        }

        vowels_diff == 0
    }
}

#[cfg(test)]
mod _1704_tests {
    use crate::_1704_determine_if_string_halves_are_alike::*;

    #[test]
    fn test1() {
        let s = "book".to_string();
        let result = Solution::halves_are_alike(s);

        assert_eq!(result, true)
    }

    #[test]
    fn test2() {
        let s = "textbook".to_string();
        let result = Solution::halves_are_alike(s);

        assert_eq!(result, false)
    }

    #[test]
    fn test3() {
        let s = "xx".to_string();
        let result = Solution::halves_are_alike(s);

        assert_eq!(result, true)
    }

    #[test]
    fn test4() {
        let s = "aa".to_string();
        let result = Solution::halves_are_alike(s);

        assert_eq!(result, true)
    }

    #[test]
    fn test5() {
        let s = "XX".to_string();
        let result = Solution::halves_are_alike(s);

        assert_eq!(result, true)
    }

    #[test]
    fn test6() {
        let s = "AA".to_string();
        let result = Solution::halves_are_alike(s);

        assert_eq!(result, true)
    }

    #[test]
    fn test7() {
        let s = "aA".to_string();
        let result = Solution::halves_are_alike(s);

        assert_eq!(result, true)
    }
}
