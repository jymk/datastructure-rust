use std::{cell::RefCell, rc::Rc};

//普通二叉树
pub struct BinaryTree<T> {
    _root: InnerBTNode<T>,
}

type InnerBTNode<T> = Option<Box<BTNode<T>>>;

pub struct BTNode<T> {
    _left: InnerBTNode<T>,
    _right: InnerBTNode<T>,
    _parent: Option<Rc<RefCell<BTNode<T>>>>,
    _val: Option<T>,
}

impl<T> BTNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            _left: None,
            _right: None,
            _parent: None,
            _val: Some(val),
        }
    }
}

impl<T> BinaryTree<T> {
    //初始化并设置根节点
    pub fn new(val: T) -> Self {
        Self {
            _root: Some(Box::new(BTNode::new(val))),
        }
    }
    pub fn get_val(node: &InnerBTNode<T>) -> Option<&T> {
        if let Some(n) = node {
            if let Some(x) = &n._val {
                return Some(x);
            }
        }
        None
    }
    //获取子节点
    pub fn get_children() {}
    //获取父节点
    pub fn get_parent() {}
    //枚举(包括该结点的所有子孙)
    pub fn enumerate() {}
    //插入
    pub fn insert(val: T) {}
    //删除
    pub fn delete() {}
    //嫁接
    pub fn graft() {}
    //修剪
    pub fn prune() {}
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self { _root: None }
    }
}
