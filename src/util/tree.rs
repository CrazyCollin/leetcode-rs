use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode{
    pub val:i32,
    pub left:Option<Rc<RefCell<TreeNode>>>,
    pub right:Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val:i32)->Self{
        TreeNode{
            val,
            left:None,
            right:None,
        }
    }
}