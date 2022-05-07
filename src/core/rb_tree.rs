use std::{cell::RefCell, ops::DerefMut, rc::Rc};

//每个节点是红的或者黑的
//根节点是黑的
//(叶子nil为黑，无用)
//若一个节点是红的，则其两个儿子不能是红的
//对每个节点，从该节点到其子孙节点的所有路径上包含相同数目的黑节点（黑高相等）
///////////////////////////////////////////////
///      2              左旋              4
/// 1        4         -->       2             5
///        3   5      <--     1  3
///                     右旋
//////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RBTree<T> {
    _root: RBInnerNode<T>,
}

type RBInnerNode<T> = Option<Rc<RefCell<RBNode<T>>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RBNode<T> {
    _left: RBInnerNode<T>,
    _right: RBInnerNode<T>,
    _parent: RBInnerNode<T>,
    _val: Option<T>,
    _color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
}

impl<T> RBTree<T> {
    pub fn new(val: T) -> Self {
        Self {
            _root: Some(Rc::new(RefCell::new(RBNode::new(val)))),
        }
    }
    pub fn insert(&mut self, val: T) {
        if let Some(x) = self._root.as_mut() {
            x.borrow_mut().insert(val);
        } else {
            self._root = Some(Rc::new(RefCell::new(RBNode::new(val))));
        }
    }
    pub fn delete(&mut self, val: &T) {
        if let Some(x) = self._root.as_mut() {
            x.borrow_mut().delete(val);
        }
    }
    //左旋
    fn _rotate_left(&mut self, node: &mut RBNode<T>) {
        if let Some(x) = self._root.as_mut() {
            node._rotate_left(x.borrow_mut().deref_mut())
        }
    }
    //右旋
    fn _rotate_right(&mut self, node: &mut RBNode<T>) {
        if let Some(x) = self._root.as_mut() {
            node._rotate_right(x.borrow_mut().deref_mut())
        }
    }
}

impl<T> RBNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            _left: None,
            _right: None,
            _parent: None,
            _val: Some(val),
            _color: Color::default(),
        }
    }
    pub fn insert(&mut self, val: T) {}
    pub fn delete(&mut self, val: &T) {}
    //左旋
    pub(crate) fn _rotate_left(&mut self, root: &mut RBNode<T>) {}
    //右旋
    pub(crate) fn _rotate_right(&mut self, root: &mut RBNode<T>) {}
}

impl<T> Default for RBTree<T> {
    fn default() -> Self {
        Self { _root: None }
    }
}

impl<T> Default for RBNode<T> {
    fn default() -> Self {
        Self {
            _left: None,
            _right: None,
            _parent: None,
            _val: None,
            _color: Color::default(),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Black
    }
}
