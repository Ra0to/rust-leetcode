use super::models::ListNode;
struct Solution;

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }

        if head.is_none() {
            return None;
        }

        let mut n = 0;
        let mut node = &mut head;
        loop {
            n += 1;
            if node.as_ref().unwrap().next.is_none() {
                break;
            }

            node = &mut node.as_mut().unwrap().next;
        }

        if k % n == 0 {
            return head;
        }

        k = n - k % n - 1;

        node = &mut head;

        while k > 0 {
            node = &mut node.as_mut().unwrap().next;
            k -= 1;
        }

        let mut new_start = node.as_mut().unwrap().next.take();

        let mut node = &mut new_start;
        loop {
            n += 1;
            if node.as_ref().unwrap().next.is_none() {
                break;
            }

            node = &mut node.as_mut().unwrap().next;
        }
        node.as_mut().unwrap().next = head;

        new_start
    }
}

#[cfg(test)]
mod _61_tests {
    use crate::_61_rotate_list::*;

    #[test]
    fn test1() {
        let head = ListNode::create(vec![1, 2, 3, 4, 5]);
        let k = 2;
        let ans = ListNode::create(vec![4, 5, 1, 2, 3]);

        let res = Solution::rotate_right(head, k);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let head = ListNode::create(vec![1, 2, 3, 4, 5]);
        let k = 0;
        let ans = ListNode::create(vec![1, 2, 3, 4, 5]);

        let res = Solution::rotate_right(head, k);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let head = ListNode::create(vec![1, 2, 3, 4, 5]);
        let k = 1;
        let ans = ListNode::create(vec![5, 1, 2, 3, 4]);

        let res = Solution::rotate_right(head, k);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let head = ListNode::create(vec![1, 2]);
        let k = 100;
        let ans = ListNode::create(vec![1, 2]);

        let res = Solution::rotate_right(head, k);

        assert_eq!(res, ans);
    }

    #[test]
    fn test5() {
        let head = ListNode::create(vec![1, 2]);
        let k = 101;
        let ans = ListNode::create(vec![2, 1]);

        let res = Solution::rotate_right(head, k);

        assert_eq!(res, ans);
    }
}
