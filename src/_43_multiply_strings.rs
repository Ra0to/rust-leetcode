pub struct Solution;

fn to_number(digits: &Vec<u8>) -> String {
    let str = digits
        .iter()
        .skip_while(|&&x| x == 0)
        .map(|&num| (num + b'0') as char)
        .collect::<String>();
    if str.is_empty() {
        return String::from("0");
    }

    str
}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let len1 = num1.len();
        let len2 = num2.len();

        let num1_digits = num1
            .chars()
            .into_iter()
            .map(|c| c as u8 - b'0')
            .collect::<Vec<u8>>();
        let num2_digits = num2
            .chars()
            .into_iter()
            .map(|c| c as u8 - b'0')
            .collect::<Vec<u8>>();

        let mut parts = vec![vec![0 as u8; len1 + 1]; len2];

        for i in 0..len2 {
            let mut carry = 0;
            let part = &mut parts[i];
            for j in (0..len1).rev() {
                let val = num1_digits[j] * num2_digits[len2 - i - 1] + carry;
                part[j + 1] = val % 10;
                carry = val / 10;
            }

            part[0] = carry;
        }

        let ans_len = len1 + len2;
        let mut ans = vec![0 as u8; ans_len];

        let mut carry = 0;

        for i in 0..ans_len {
            let mut sum: u16 = carry;
            for part_index in 0..=i.min(parts.len() - 1) {
                let digit_index = i - part_index;
                let part = &mut parts[part_index];
                if digit_index < part.len() {
                    sum += part[part.len() - digit_index - 1] as u16;
                }
            }

            ans[ans_len - i - 1] = (sum % 10) as u8;
            carry = sum / 10;
        }

        to_number(&ans)
    }
}

#[cfg(test)]
mod _43_tests {
    use crate::_43_multiply_strings::*;

    #[test]
    fn test1() {
        let num1 = String::from("2");
        let num2 = String::from("3");
        let ans = String::from("6");

        let res = Solution::multiply(num1, num2);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let num1 = String::from("23");
        let num2 = String::from("12");
        let ans = String::from("276");

        let res = Solution::multiply(num1, num2);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let num1 = String::from("999");
        let num2 = String::from("12");
        let ans = String::from("11988");

        let res = Solution::multiply(num1, num2);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let num1 = String::from("123");
        let num2 = String::from("456");
        let ans = String::from("56088");

        let res = Solution::multiply(num1, num2);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let num1 = String::from("12818237491283479812733");
        let num2 = String::from("0");
        let ans = String::from("0");

        let res = Solution::multiply(num1, num2);

        assert_eq!(res, ans);
    }

    #[test]
    fn test6() {
        let num1 = String::from("12818237491283479812733");
        let num2 = String::from("1000");
        let ans = String::from("12818237491283479812733000");

        let res = Solution::multiply(num1, num2);

        assert_eq!(res, ans);
    }

    #[test]
    fn test7() {
        let num1 = String::from("4");
        let num2 = String::from("1001");
        let ans = String::from("4004");

        let res = Solution::multiply(num1, num2);

        assert_eq!(res, ans);
    }

    #[test]
    fn test8() {
        let num1 = String::from("23");
        let num2 = String::from("11");
        let ans = String::from("253");

        let res = Solution::multiply(num1, num2);

        assert_eq!(res, ans);
    }

    #[test]
    fn test9() {
        let num1 = String::from("233");
        let num2 = String::from("1001");
        let ans = String::from("233233");

        let res = Solution::multiply(num1, num2);

        assert_eq!(res, ans);
    }

    #[test]
    fn test10() {
        // Testcase #290
        let num1 = String::from("6964594125027226699998128707627435007621143975736747759751");
        let num2 = String::from("333791918659904899647584436187733004125181273682766434");
        let ans = String::from("2324725235680339589575434145174345450376468286967007130548581359508676923461769510883584014763890133705678997934");

        let res = Solution::multiply(num1, num2);

        assert_eq!(res, ans);
    }
}
