#[cfg(test)]
use super::ListNode;

#[cfg(test)]
impl ListNode {
    pub fn create(vec: Vec<i32>) -> Option<Box<ListNode>> {
        if vec.is_empty() {
            return None;
        }

        Some(Box::new(ListNode::from(vec)))
    }
}

#[cfg(test)]
impl From<Vec<i32>> for ListNode {
    fn from(vec: Vec<i32>) -> Self {
        *vec.iter()
            .rev()
            .fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
            .unwrap()
    }
}

#[cfg(test)]
mod list_node_tests {
    use crate::models::ListNode;

    #[test]
    fn test_linked_list_creation() {
        let list = ListNode::create(vec![1, 2, 3, 4, 5]);

        let mut node = &list;
        assert_eq!(node.as_ref().map(|x| x.val), Some(1));
        node = &node.as_ref().unwrap().next;
        assert_eq!(node.as_ref().map(|x| x.val), Some(2));
        node = &node.as_ref().unwrap().next;
        assert_eq!(node.as_ref().map(|x| x.val), Some(3));
        node = &node.as_ref().unwrap().next;
        assert_eq!(node.as_ref().map(|x| x.val), Some(4));
        node = &node.as_ref().unwrap().next;
        assert_eq!(node.as_ref().map(|x| x.val), Some(5));
        node = &node.as_ref().unwrap().next;
        assert!(node.is_none());
    }

    #[test]
    fn test_linked_list_cut() {
        let mut list = ListNode::create(vec![1, 2, 3, 4, 5]);

        let mut node = &mut list;
        assert_eq!(node.as_ref().map(|x| x.val), Some(1));
        node = &mut node.as_mut().unwrap().next;
        assert_eq!(node.as_ref().map(|x| x.val), Some(2));
        node = &mut node.as_mut().unwrap().next;
        assert_eq!(node.as_ref().map(|x| x.val), Some(3));
        let last = node.as_mut().unwrap();
        let second_list = last.next.take();

        let mut node = &list;
        assert_eq!(node.as_ref().map(|x| x.val), Some(1));
        node = &node.as_ref().unwrap().next;
        assert_eq!(node.as_ref().map(|x| x.val), Some(2));
        node = &node.as_ref().unwrap().next;
        assert_eq!(node.as_ref().map(|x| x.val), Some(3));
        node = &node.as_ref().unwrap().next;
        assert!(node.is_none());

        let mut node: &Option<Box<ListNode>> = &second_list;
        assert_eq!(node.as_ref().map(|x| x.val), Some(4));
        node = &node.as_ref().unwrap().next;
        assert_eq!(node.as_ref().map(|x| x.val), Some(5));
        node = &node.as_ref().unwrap().next;
        assert!(node.is_none());
    }

    #[test]
    fn test_linked_lists_union() {
        let mut first = ListNode::create(vec![1, 2, 3]);
        let second = ListNode::create(vec![4, 5]);

        let mut node = &mut first;
        node = &mut node.as_mut().unwrap().next;
        node = &mut node.as_mut().unwrap().next;
        node.as_mut().unwrap().next = second.clone();

        let mut node = &first;
        assert_eq!(node.as_ref().map(|x| x.val), Some(1));
        node = &node.as_ref().unwrap().next;
        assert_eq!(node.as_ref().map(|x| x.val), Some(2));
        node = &node.as_ref().unwrap().next;
        assert_eq!(node.as_ref().map(|x| x.val), Some(3));
        node = &node.as_ref().unwrap().next;
        assert_eq!(node.as_ref().map(|x| x.val), Some(4));
        node = &node.as_ref().unwrap().next;
        assert_eq!(node.as_ref().map(|x| x.val), Some(5));
        node = &node.as_ref().unwrap().next;
        assert!(node.is_none());
    }
}
