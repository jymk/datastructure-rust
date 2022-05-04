use std::{cell::RefCell, rc::Rc};

use crate::common::pos::Pos;

//普通二叉树
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryTree<T> {
    _root: InnerBTNode<T>,
}

type InnerBTNode<T> = Option<Box<BTNode<T>>>;

#[derive(Debug, Clone, Eq, PartialEq)]
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
    //先序遍历
    pub fn pre_order(&self) {}
    //中序遍历
    pub fn in_order(&self) {}
    //后续遍历
    pub fn post_order(&self) {}
}
impl<T: PartialEq + Eq> BTNode<T> {
    //删除
    pub fn delete(&mut self, node: &Self, pos: Pos) {
        //获取父节点，对比父节点的左和右节点，哪个相等就删除哪个，并赋予父节点的_left或_right为其pos节点
        //上述无法实现，若被删除节点左右节点都有值，另一个节点要放到哪里
        // match &self._parent {
        //     Some(p) => {
        //         let p_clone = p.clone();
        //         //若左边为空，说明右边肯定不为空
        //         if let Some(l) = &p.borrow_mut()._left {
        //             //若左边不等，说明右边也不为空
        //             if *self == **l {
        //                 p_clone.borrow_mut()._left = None;
        //             }
        //         } else {
        //             p_clone.borrow_mut()._right = None;
        //         }
        //     }
        //     None => return,
        // }
    }
    //修剪当前节点
    pub fn prune(&mut self) {
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
impl<T: Clone> BTNode<T> {
    //获取父节点
    pub fn get_parent(&self) -> Option<Self> {
        match &self._parent {
            Some(v) => Some(v.borrow().clone()),
            None => None,
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
    //先序遍历
    pub fn pre_order(&self) {
        match &self._root {
            Some(v) => v.pre_order(),
            None => return,
        }
    }
    //中序遍历
    pub fn in_order(&self) {
        match &self._root {
            Some(v) => v.in_order(),
            None => return,
        }
    }
    //后续遍历
    pub fn post_order(&self) {
        match &self._root {
            Some(v) => v.post_order(),
            None => return,
        }
    }
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self { _root: None }
    }
}
