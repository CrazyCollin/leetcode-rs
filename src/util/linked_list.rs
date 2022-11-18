#[derive(Debug,PartialEq,Eq)]
pub struct ListNode{
    pub val:i32,
    pub next:Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val:i32)->ListNode{
        ListNode{
            val,
            next: None,
        }
    }
}


