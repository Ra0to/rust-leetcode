use crate::models::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ans = ListNode::new(-1);

        let mut node = &mut dbg!(head);
        while node.is_some() && node.as_ref().unwrap().next.is_some() {
            while node.is_some()
                && node.as_ref().unwrap().next.is_some()
                && node.as_ref().unwrap().val == node.as_mut().unwrap().next.as_ref().unwrap().val
            {
                node = &mut dbg!(node).as_mut().unwrap().next;
            }

            if node.is_none() {
                break;
            }

            node = &mut node.as_mut().unwrap().next;
            ans.next = Some(Box::new(ListNode::new(node.as_ref().unwrap().val)));
        }

        ans.next
    }
}

#[cfg(test)]
mod _82_tests {
    use crate::_82_remove_duplicates_from_sorted_list_ii::*;

    #[test]
    fn test1() {
        let head = ListNode::create(vec![1, 2, 3, 3, 4, 4, 5]);
        let ans = ListNode::create(vec![1, 2, 5]);
        let res = Solution::delete_duplicates(head);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let head = ListNode::create(vec![1, 1, 1, 2, 3]);
        let ans = ListNode::create(vec![1, 2, 3]);
        let res = Solution::delete_duplicates(head);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let head = None;
        let ans = None;
        let res = Solution::delete_duplicates(head);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let head: Option<Box<ListNode>> = ListNode::create(vec![1]);
        let ans = ListNode::create(vec![1]);
        let res = Solution::delete_duplicates(head);

        assert_eq!(res, ans);
    }
}
