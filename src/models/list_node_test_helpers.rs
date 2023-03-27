#[cfg(test)]
use super::ListNode;

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
impl Into<Vec<i32>> for ListNode {
    fn into(self) -> Vec<i32> {
        self.into_iter().collect()
    }
}

#[cfg(test)]
impl Iterator for ListNode {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.take();
        match next {
            Some(boxed) => {
                let node = *boxed;
                self.next = node.next;
                Some(node.val)
            }
            None => None,
        }
    }
}
