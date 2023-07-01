struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut index: i32 = (digits.len() - 1) as i32;
        let mut carry: i32 = 1;

        while index >= 0 && carry > 0 {
            let sum = digits[index as usize] + carry as i32;
            digits[index as usize] = sum % 10;
            carry = sum / 10;
            index -= 1;
        }

        if carry > 0 {
            digits.insert(0, carry as i32);
        }
        
        digits
    }
}

#[cfg(test)]
mod _66_tests {
    use crate::_66_plus_one::*;

    #[test]
    fn test1() {
        let digits = vec![1, 2, 3];
        let ans = vec![1, 2, 4];

        let res = Solution::plus_one(digits);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let digits = vec![4, 3, 2, 1];
        let ans = vec![4, 3, 2, 2];

        let res = Solution::plus_one(digits);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let digits = vec![9];
        let ans = vec![1, 0];

        let res = Solution::plus_one(digits);

        assert_eq!(res, ans);
    }
}
