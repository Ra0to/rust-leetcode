pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.chars()
            .into_iter()
            .rev()
            .skip_while(|c| *c == ' ')
            .take_while(|c| *c != ' ')
            .count() as i32
    }
}

#[cfg(test)]
mod _58_tests {
    use crate::_58_length_of_last_word::*;

    #[test]
    fn test1() {
        let s = String::from("Hello World");
        let ans = 5;

        let res = Solution::length_of_last_word(s);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let s = String::from("   fly me   to   the moon  ");
        let ans = 4;

        let res = Solution::length_of_last_word(s);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let s = String::from("luffy is still joyboy");
        let ans = 6;

        let res = Solution::length_of_last_word(s);

        assert_eq!(res, ans);
    }
}
