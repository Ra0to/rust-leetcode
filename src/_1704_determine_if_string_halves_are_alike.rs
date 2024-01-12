pub struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let vowels: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut vowels_count = 0;
        let chars: Vec<char> = s.chars().collect();

        let len = s.len();
        for i in 0..(len / 2) {
            vowels_count += if vowels.contains(&chars[i]) { 1 } else { 0 };
            vowels_count += if vowels.contains(&chars[len - i - 1]) {
                -1
            } else {
                0
            };
        }

        vowels_count == 0
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
