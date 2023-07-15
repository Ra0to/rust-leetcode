use crate::models::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node = &mut head;
        while node.is_some() && node.as_ref().unwrap().next.is_some() {
            if node.as_mut().unwrap().next.as_ref().unwrap().val == node.as_ref().unwrap().val {
                node.as_mut().unwrap().next =
                    node.as_mut().unwrap().next.as_mut().unwrap().next.take();
                continue;
            }

            node = &mut node.as_mut().unwrap().next;
        }

        head
    }
}

#[cfg(test)]
mod _83_tests {
    use crate::_83_remove_duplicates_from_sorted_list::*;

    #[test]
    fn test1() {
        let head = ListNode::create(vec![1, 1, 2]);
        let ans = ListNode::create(vec![1, 2]);

        let res = Solution::delete_duplicates(head);

        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let head = ListNode::create(vec![1, 1, 2, 3, 3]);
        let ans = ListNode::create(vec![1, 2, 3]);

        let res = Solution::delete_duplicates(head);

        assert_eq!(res, ans);
    }

    #[test]
    fn test3() {
        let head = ListNode::create(vec![0, 0, 0, 0, 0]);
        let ans = ListNode::create(vec![0]);

        let res = Solution::delete_duplicates(head);

        assert_eq!(res, ans);
    }

    #[test]
    fn test4() {
        let head = ListNode::create(vec![]);
        let ans = ListNode::create(vec![]);

        let res = Solution::delete_duplicates(head);

        assert_eq!(res, ans);
    }
}
