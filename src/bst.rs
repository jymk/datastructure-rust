use std::{cell::RefCell, fmt::Debug, rc::Rc};

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
    //嫁接node到self的pos位置
    pub fn graft(&mut self, node: &Self, pos: Pos) {}
}

impl<T: PartialEq> BSTNode<T> {
    //修剪掉当前节点
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

impl<T: Clone + Ord> BSTNode<T> {
    fn _insert_node(&mut self, node: Self) {
        match node._val.cmp(&self._val) {
            std::cmp::Ordering::Equal => return,
            std::cmp::Ordering::Less => match self._left.as_mut() {
                Some(x) => x._insert_node(node),
                None => self._left = Some(Box::new(node)),
            },
            std::cmp::Ordering::Greater => match self._right.as_mut() {
                Some(x) => x._insert_node(node),
                None => self._right = Some(Box::new(node)),
            },
        }
    }
}
impl<T: Debug> BSTNode<T> {
    //枚举(包括该结点的所有子孙)
    pub fn enumerate(&self) {
        print!("{:?} => ", self._val);
        if let Some(v) = &self._left {
            v.enumerate();
        }
        if let Some(v) = &self._right {
            v.enumerate();
        }
    }
}
impl<T: Clone> BSTNode<T> {
    fn _get_children(&self, res: &mut Vec<Self>) {
        if let Some(v) = &self._left {
            v._get_children(res);
        }
        if let Some(v) = &self._right {
            v._get_children(res);
        }
        res.push((*self).clone());
    }
    //获取子节点
    pub fn get_children(&self) -> Vec<Self> {
        let mut res = vec![];
        self._get_children(&mut res);
        res
    }
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
impl<T: Debug> BST<T> {
    pub fn enumerate(&self) {
        if let Some(x) = &self._root {
            x.enumerate();
        }
        println!("None");
    }
}
impl<T: Clone + Ord> BST<T> {
    //插入node到self的pos位置
    pub fn insert(&mut self, val: T) {
        let node = BSTNode::new(val);
        if let Some(x) = self._root.as_mut() {
            x._insert_node(node);
        } else {
            self._root = Some(Box::new(node));
        }
    }
    //删除
    pub fn delete(&mut self, val: &T) {
        let mut datas = self.pre_order();
        //排序方便找到合适的根节点
        datas.sort();
        //新根节点数据
        let len = datas.len();
        let r_data = datas[len / 2].clone();
        //删除val元素和根节点数据(即保留不等于val并且不等于r_data的)
        datas.retain(|x| x != val && x != &r_data);
        //若没有此元素
        if datas.len() == len {
            return;
        }
        let mut new_tree = Self::new(r_data);
        for data in datas {
            new_tree.insert(data);
        }
        //使用新生成的树复制给self(相当于重排序)
        *self = new_tree;
    }
}
impl<T: PartialEq> BST<T> {
    pub fn prune(&mut self) {
        // self._root
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

#[test]
fn test() {
    let mut bst = BST::new(10);
    bst.insert(1);
    bst.insert(12);
    bst.insert(11);
    bst.delete(&9);
    bst.enumerate();
    // let datas = bst.post_order();
    // println!("datas: {:?}", datas);
}
