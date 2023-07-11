pub struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        // Convert K to factoradic number. Then to permutation
        let n = n as usize;
        let mut nums = (1..=n as u8).collect::<Vec<u8>>();
        let mut facts = vec![1 as i32; n - 1];

        for i in 1..(n - 1) {
            facts[i] = facts[i - 1] * (i + 1) as i32;
        }

        let mut ans = String::new();

        for i in 0..(n - 1) {
            let fact = facts[n - i - 2];
            let index = (k - 1) / fact;
            k -= index * fact;
            let index = index as usize;
            ans += &nums[index].to_string();
            nums.remove(index);
        }
        ans += &nums[0].to_string();

        ans
    }
}

#[cfg(test)]
mod _60_tests {
    use crate::_60_permutation_sequence::*;

    #[test]
    fn test1() {
        let n = 3;
        let k = 3;
        let ans = "213";

        let res = Solution::get_permutation(n, k);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let n = 4;
        let k = 9;
        let ans = "2314";

        let res = Solution::get_permutation(n, k);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let n = 3;
        let k = 1;
        let ans = "123";

        let res = Solution::get_permutation(n, k);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let n = 4;
        let k = 7;
        let ans = "2134";

        let res = Solution::get_permutation(n, k);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let n = 4;
        let k = 6;
        let ans = "1432";

        let res = Solution::get_permutation(n, k);

        assert_eq!(res, ans);
    }

    #[test]
    fn test6() {
        let n = 9;
        //
        let k = 362880;
        let ans = "987654321";

        let res = Solution::get_permutation(n, k);

        assert_eq!(res, ans);
    }
}
