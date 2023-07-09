// Original traits:
// #[derive(PartialEq, Eq, Clone, Debug)]
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(self.val);

        let mut next = &self.next;

        while let Some(node) = next {
            vec.push(node.val);
            next = &node.next;
        }

        f.debug_struct("ListNode").field("nodes", &vec).finish()
    }
}
