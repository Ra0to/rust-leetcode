pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();

        for part in path.split("/") {
            match part {
                "" | "." => continue,
                ".." => {
                    stack.pop();
                }
                _ => stack.push(part),
            };
        }

        String::from("/") + &stack.join("/")
    }
}

#[cfg(test)]
mod _71_tests {
    use crate::_71_simplify_path::*;

    #[test]
    fn test1() {
        let path = String::from("/home/");
        let ans = String::from("/home");

        let res = Solution::simplify_path(path);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let path = String::from("/../");
        let ans = String::from("/");

        let res = Solution::simplify_path(path);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let path = String::from("/home//foo/");
        let ans = String::from("/home/foo");

        let res = Solution::simplify_path(path);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let path = String::from("/1//2/3/sk.ip/sk..ip/../../");
        let ans = String::from("/1/2/3");

        let res = Solution::simplify_path(path);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let path = String::from("///...///skip///skip///..//.//..///");
        let ans = String::from("/...");

        let res = Solution::simplify_path(path);

        assert_eq!(res, ans);
    }

    #[test]
    fn test6() {
        let path = String::from("/.");
        let ans = String::from("/");

        let res = Solution::simplify_path(path);

        assert_eq!(res, ans);
    }

    #[test]
    fn test7() {
        let path = String::from("/home/user/me");
        let ans = String::from("/home/user/me");

        let res = Solution::simplify_path(path);

        assert_eq!(res, ans);
    }
}
