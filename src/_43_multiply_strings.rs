struct Solution;

const ZERO: u8 = '0' as u8;

fn get_digit(c: char) -> u8 {
    c as u8 - ZERO
}

fn to_number(digits: &Vec<u8>) -> String {
    let str = digits
        .iter()
        .skip_while(|&&x| x == 0)
        .map(|&num| (num + ZERO) as char)
        .collect::<String>();
    if str.is_empty() {
        return String::from("0");
    }

    str
}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1_digits = num1
            .chars()
            .into_iter()
            .map(|c| get_digit(c))
            .collect::<Vec<u8>>();
        let num2_digits = num2
            .chars()
            .into_iter()
            .map(|c| get_digit(c))
            .collect::<Vec<u8>>();

        let mut parts = vec![vec![0 as u8; num1.len()]; num2.len()];
        let mut max_part_len = 0 as usize;

        for i in (0..num2.len()).rev() {
            dbg!(&i);
            let mut carry = 0;
            let part = &mut parts[i];
            for j in (0..num1.len()).rev() {
                let val = num1_digits[j] * num2_digits[i] + carry;
                part[j] = val % 10;
                carry = val / 10;
            }

            if carry > 0 {
                part.insert(0, carry);
            }

            for _ in 0..(num2.len() - i - 1) {
                part.push(0);
            }

            max_part_len = max_part_len.max(part.len());

            dbg!(to_number(part));
        }

        let mut ans = vec![0 as u8; max_part_len];

        let mut carry = 0;

        for i in 0..max_part_len {
            let mut sum: u16 = carry;
            for part in parts.iter() {
                if i < part.len() {
                    sum += part[part.len() - i - 1] as u16;
                }
            }

            ans[max_part_len - i - 1] = (sum % 10) as u8;
            carry = sum / 10;
        }

        if carry > 0 {
            ans.insert(0, carry as u8);
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
