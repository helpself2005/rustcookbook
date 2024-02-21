#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}


impl ListNode {
    pub(crate) fn new(val: i32) -> Self {
        ListNode{ val, next:None}
    }
}