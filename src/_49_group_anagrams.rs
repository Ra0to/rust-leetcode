use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ans = HashMap::<String, Vec<String>>::new();

        for (x, key) in strs.into_iter().map(|x| Self::get_sorted_string(x)) {
            if !ans.contains_key(&key) {
                ans.insert(key, vec![x]);
                continue;
            }

            let collection = ans.get_mut(&key).unwrap();

            collection.push(x);
        }

        ans.into_iter().map(|x| x.1).collect()
    }

    fn get_sorted_string(str: String) -> (String, String) {
        let mut bytes = str.as_bytes().to_vec();
        bytes.sort();
        (str, String::from_utf8(bytes).unwrap())
    }
}

#[cfg(test)]
mod _49_tests {
    use crate::_49_group_anagrams::*;

    #[test]
    fn test1() {
        let strs: Vec<String> = (vec!["eat", "tea", "tan", "ate", "nat", "bat"])
            .iter()
            .map(|x| String::from(*x))
            .collect();
        let mut ans = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];

        let mut res = Solution::group_anagrams(strs);
        ans.iter_mut().for_each(|vec| vec.sort_unstable());
        ans.sort_unstable();

        res.iter_mut().for_each(|vec| vec.sort_unstable());
        res.sort_unstable();

        assert_eq!(res, ans);
    }
}
