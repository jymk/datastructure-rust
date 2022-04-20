use std::{cell::RefCell, rc::Rc};

//普通二叉搜索树
struct BST<T> {
    _left: Option<Rc<RefCell<T>>>,
    _right: Option<Rc<RefCell<T>>>,
    _val: Option<T>,
}

impl<T> BST<T> {}

impl<T> Default for BST<T> {
    fn default() -> Self {
        Self {
            _left: None,
            _right: None,
            _val: None,
        }
    }
}
