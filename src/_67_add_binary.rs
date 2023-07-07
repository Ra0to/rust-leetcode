struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_chars = a.chars().collect::<Vec<char>>();
        let b_chars = b.chars().collect::<Vec<char>>();

        let mut carry = 0;

        let len = a.len().max(b.len());
        let mut ans = vec!['0'; len];

        for i in 0..len {
            let x = if i >= a.len() {
                0
            } else {
                a_chars[a.len() - i - 1] as u8 - '0' as u8
            };
            let y = if i >= b.len() {
                0
            } else {
                b_chars[b.len() - i - 1] as u8 - '0' as u8
            };
            let sum = x + y + carry;
            ans[len - i - 1] = (sum % 2 + '0' as u8) as char;

            carry = sum / 2;
        }

        if carry > 0 {
            ans.insert(0, '1');
        }

        ans.iter().collect()
    }
}

#[cfg(test)]
mod _67_tests {
    use crate::_67_add_binary::*;

    #[test]
    fn test1() {
        let a = String::from("11");
        let b = String::from("1");
        let ans = String::from("100");

        let res = Solution::add_binary(a, b);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let a = String::from("1010");
        let b = String::from("1011");
        let ans = String::from("10101");

        let res = Solution::add_binary(a, b);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let a = String::from("0");
        let b = String::from("1011");
        let ans = String::from("1011");

        let res = Solution::add_binary(a, b);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let a = String::from("101");
        let b = String::from("11110101110110");
        let ans = String::from("11110101111011");

        let res = Solution::add_binary(a, b);

        assert_eq!(res, ans);
    }
}
