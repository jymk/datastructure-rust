use std::{cell::RefCell, rc::Rc};

use crate::common::pos::Pos;

//二叉搜索树
#[derive(Debug, Clone, PartialEq, Eq)]
struct BST<T> {
    _root: InnerBSTNode<T>,
}

type InnerBSTNode<T> = Option<Box<BSTNode<T>>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BSTNode<T> {
    _left: InnerBSTNode<T>,
    _right: InnerBSTNode<T>,
    _parent: Option<Rc<RefCell<BSTNode<T>>>>,
    _val: Option<T>,
}

impl<T> BSTNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            _left: None,
            _right: None,
            _parent: None,
            _val: Some(val),
        }
    }
    pub fn get_val(&self) -> Option<&T> {
        match &self._val {
            Some(v) => Some(v),
            None => None,
        }
    }
    //获取子节点
    pub fn get_children(&self) {}
    //枚举(包括该结点的所有子孙)
    pub fn enumerate(&self) {}
    //插入node到self的pos位置
    pub fn insert(&mut self, node: &Self, pos: Pos) {}
    //嫁接node到self的pos位置
    pub fn graft(&mut self, node: &Self, pos: Pos) {}
}

impl<T: PartialEq + Eq> BSTNode<T> {
    //修剪当前节点
    pub fn prune(&self) {
        //获取父节点，对比父节点的左和右节点，哪个相等就删除哪个
        match &self._parent {
            Some(p) => {
                let p_clone = p.clone();
                //若左边为空，说明右边肯定不为空
                if let Some(l) = &p.borrow_mut()._left {
                    //若左边不等，说明右边也不为空
                    if *self == **l {
                        p_clone.borrow_mut()._left = None;
                    }
                } else {
                    p_clone.borrow_mut()._right = None;
                }
            }
            None => return,
        }
    }
}

impl<T: Clone + PartialEq + Eq> BSTNode<T> {
    //删除，删除后取pos位置为子节点
    pub fn delete(&mut self, pos: Pos) {
        //获取父节点，对比父节点的左和右节点，哪个相等就删除哪个，并赋予父节点的_left或_right为其pos节点
        //上述无法实现，若被删除节点左右节点都有值，另一个节点重排序到新_left或_right
        match &self._parent {
            Some(p) => {
                let p_clone = p.clone();
                //若左边为空，说明右边肯定不为空
                if let Some(l) = &mut p.borrow_mut()._left {
                    //若左边不等，说明右边也不为空
                    if *self == **l {
                        match pos {
                            Pos::Left => {
                                p_clone.borrow_mut()._left = l._left.clone();
                                if let Some(ll) = &mut l._left {
                                    ll._parent = Some(p_clone);
                                    // ll._right = l._right.clone();
                                }
                            }
                            Pos::Right => {
                                p_clone.borrow_mut()._left = l._right.clone();
                                if let Some(lr) = &mut l._right {
                                    lr._parent = Some(p_clone);
                                    // lr._left = l._left.clone();
                                }
                            }
                        }
                    }
                } else {
                    if let Some(r) = &mut p.borrow_mut()._right {
                        match pos {
                            Pos::Left => {
                                p_clone.borrow_mut()._right = r._left.clone();
                            }
                            Pos::Right => {}
                        }
                    }
                }
            }
            None => return,
        }
    }
}

impl<T: Clone> BSTNode<T> {
    //获取父节点
    pub fn get_parent(&self) -> Option<Self> {
        match &self._parent {
            Some(v) => Some(v.borrow().clone()),
            None => None,
        }
    }
    //先序遍历
    pub fn pre_order(&self, res: &mut Vec<T>) {
        if let Some(v) = &self._val {
            res.push(v.clone());
        }
        if let Some(v) = &self._left {
            v.pre_order(res);
        }
        if let Some(v) = &self._right {
            v.pre_order(res);
        }
    }
    //中序遍历
    pub fn in_order(&self, res: &mut Vec<T>) {
        if let Some(v) = &self._left {
            v.pre_order(res);
        }
        if let Some(v) = &self._val {
            res.push(v.clone());
        }
        if let Some(v) = &self._right {
            v.pre_order(res);
        }
    }
    //后续遍历
    pub fn post_order(&self, res: &mut Vec<T>) {
        if let Some(v) = &self._left {
            v.pre_order(res);
        }
        if let Some(v) = &self._right {
            v.pre_order(res);
        }
        if let Some(v) = &self._val {
            res.push(v.clone());
        }
    }
}

impl<T> BST<T> {
    //初始化并设置根节点
    pub fn new(val: T) -> Self {
        Self {
            _root: Some(Box::new(BSTNode::new(val))),
        }
    }
}
impl<T: Clone> BST<T> {
    //先序遍历
    pub fn pre_order(&self) -> Vec<T> {
        let mut res = vec![];
        if let Some(v) = &self._root {
            v.pre_order(&mut res);
        }
        res
    }
    //中序遍历
    pub fn in_order(&self) -> Vec<T> {
        let mut res = vec![];
        if let Some(v) = &self._root {
            v.in_order(&mut res);
        }
        res
    }
    //后续遍历
    pub fn post_order(&self) -> Vec<T> {
        let mut res = vec![];
        if let Some(v) = &self._root {
            v.post_order(&mut res);
        }
        res
    }
}

impl<T> Default for BST<T> {
    fn default() -> Self {
        Self { _root: None }
    }
}
