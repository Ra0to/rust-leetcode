use std::{collections::HashMap, vec};

struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let chars = s.chars().collect::<Vec<char>>();

        let word_len = words[0].len();
        let words_cnt = words.len();
        let concat_len = word_len * words_cnt;

        if s.len() < concat_len {
            return vec![];
        }

        let mut uniq_words_indexes = HashMap::<String, usize>::new();
        let mut uniq_words = Vec::<&str>::new();
        let mut uniq_words_counts = Vec::<usize>::new();

        for word in words.iter() {
            if !uniq_words_indexes.contains_key(word) {
                uniq_words.push(word);
                uniq_words_counts.push(0);
                uniq_words_indexes.insert(word.to_string(), uniq_words.len() - 1);
            }

            uniq_words_counts[uniq_words_indexes[word]] += 1;
        }

        let mut starts = vec![None as Option<usize>; s.len()];
        for (index, chars_substr) in chars.windows(word_len).enumerate() {
            // IMPROVE: implement faster string comparing with hashes
            let substr = chars_substr.into_iter().collect::<String>();

            for (uniq_word_index, word) in uniq_words.iter().enumerate() {
                if substr != **word {
                    continue;
                }

                starts[index] = Some(uniq_word_index);
                break;
            }
        }

        let mut ans = Vec::<i32>::new();

        let mut left = 0 as usize;
        let mut right = concat_len - word_len;

        let mut words_in_substr = vec![0 as usize; uniq_words.len()];
        let mut words_in_substr_cnt = 0 as usize;

        while right <= s.len() - word_len {
            for index in 0..words_in_substr.len() {
                words_in_substr[index] = 0;
            }
            words_in_substr_cnt = 0;

            for index in (left..=right).step_by(word_len) {
                if let Some(uniq_word_index) = starts[index] {
                    words_in_substr[uniq_word_index] += 1;
                    words_in_substr_cnt += 1;
                }
            }

            if words_in_substr_cnt == words_cnt {
                let mut is_wrong_count = false;

                for index in 0..uniq_words.len() {
                    if words_in_substr[index] != uniq_words_counts[index] {
                        is_wrong_count = true;
                        break;
                    }
                }

                if !is_wrong_count {
                    ans.push(left as i32);
                }
            }
            
            left += 1;
            right += 1;
        }

        ans
    }
}

#[cfg(test)]
mod _30_tests {
    use crate::_30_substring_with_concatenation_of_all_words::*;

    #[test]
    fn test1() {
        let s = String::from("barfoothefoobarman");
        let words = vec![String::from("foo"), String::from("bar")];
        let ans = vec![0, 9];

        let res = Solution::find_substring(s, words);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let s = String::from("wordgoodgoodgoodbestword");
        let words = vec![
            String::from("word"),
            String::from("good"),
            String::from("best"),
            String::from("word"),
        ];
        let ans = vec![];

        let res = Solution::find_substring(s, words);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let s = String::from("barfoofoobarthefoobarman");
        let words = vec![
            String::from("bar"),
            String::from("foo"),
            String::from("the"),
        ];
        let ans = vec![6, 9, 12];

        let res = Solution::find_substring(s, words);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let s = String::from("wordgoodgoodgoodbestword");
        let words = vec![
            String::from("word"),
            String::from("good"),
            String::from("best"),
            String::from("good"),
        ];
        let ans = vec![8];

        let res = Solution::find_substring(s, words);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let s = String::from("a");
        let words = vec![String::from("a"), String::from("a")];
        let ans = vec![];

        let res = Solution::find_substring(s, words);

        assert_eq!(res, ans);
    }

    #[test]
    fn test6() {
        let s = String::from("lingmindraboofooowingdingbarrwingmonkeypoundcake");
        let words = vec!["fooo", "barr", "wing", "ding", "wing"]
            .iter()
            .map(|str| str.to_string())
            .collect();
        let ans = vec![13];

        let res = Solution::find_substring(s, words);

        assert_eq!(res, ans);
    }

    #[test]
    fn test7() {
        let s = String::from("ababababab");
        let words = vec!["ababa", "babab"]
            .iter()
            .map(|str| str.to_string())
            .collect();
        let ans = vec![0];

        let res = Solution::find_substring(s, words);

        assert_eq!(res, ans);
    }

    #[test]
    fn test8() {
        let s = String::from("abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababab");
        let words = vec![
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba", "ab", "ba",
            "ab", "ba", "ab", "ba",
        ]
        .into_iter()
        .map(|str| String::from(str))
        .collect();
        let ans = vec![];

        let res = Solution::find_substring(s, words);

        assert_eq!(res, ans);
    }
}
