use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::common::node::{BoxEntity, BoxNode};

//二叉搜索树
#[derive(Debug, Clone, PartialEq, Eq)]
struct BST<T> {
    _root: InnerBSTNode<T>,
}

type InnerBSTNode<T> = Option<Box<BSTNode<T>>>;

#[derive(Clone, Eq, PartialEq)]
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
    pub fn new_opt(opt_val: Option<T>) -> Self {
        Self {
            _left: None,
            _right: None,
            _parent: None,
            _val: opt_val,
        }
    }
    pub fn get_val(&self) -> Option<&T> {
        match &self._val {
            Some(v) => Some(v),
            None => None,
        }
    }
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
    //嫁接node
    pub fn graft(&mut self, node: Self) {
        let children = node.get_children();
        for child in children {
            self._insert_node(child);
        }
    }
    fn _insert_node(&mut self, mut node: Self) {
        match node._val.cmp(&self._val) {
            std::cmp::Ordering::Equal => return,
            std::cmp::Ordering::Less => match self._left.as_mut() {
                Some(x) => x._insert_node(node),
                None => {
                    node._parent = Some(Rc::new(RefCell::new(self.clone())));
                    self._left = Some(Box::new(node));
                }
            },
            std::cmp::Ordering::Greater => match self._right.as_mut() {
                Some(x) => x._insert_node(node),
                None => {
                    node._parent = Some(Rc::new(RefCell::new(self.clone())));
                    self._right = Some(Box::new(node));
                }
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
        res.push(BSTNode::new_opt(self._val.clone()));
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
    //嫁接
    pub fn graft(&mut self, node: BSTNode<T>) {
        if let Some(x) = self._root.as_mut() {
            x.graft(node);
        }
    }
    //插入数据
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

impl<T: Ord> BoxNode<T> for BSTNode<T> {
    type U = Self;

    fn get_node(&self, t: &Option<&T>) -> Option<&Self::U> {
        match t.cmp(&self._val.as_ref()) {
            std::cmp::Ordering::Equal => Some(&self),
            std::cmp::Ordering::Less => {
                if let Some(x) = &self._left {
                    x.get_node(t)
                } else {
                    None
                }
            }
            std::cmp::Ordering::Greater => {
                if let Some(x) = &self._right {
                    x.get_node(t)
                } else {
                    None
                }
            }
        }
    }
}
impl<T: Ord> BoxEntity<T> for BST<T> {
    type U = BSTNode<T>;

    fn get_node(&self, t: &T) -> Option<&Self::U> {
        if let Some(x) = &self._root {
            x.get_node(&Some(t))
        } else {
            None
        }
    }
}

//只打印parent
impl<T: Debug> Debug for BSTNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} => ", self._val)?;
        if let Some(x) = &self._parent {
            return Debug::fmt(&x.borrow(), f);
        }
        write!(f, "None")
    }
}
//打印子节点
impl<T: Debug> Display for BSTNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} => ", self._val)?;
        //打印父节点，需注释下面
        // if let Some(x) = &self._parent {
        //     return Display::fmt(&x.borrow(), f);
        // }
        //打印左右节点，需注释上面
        if let Some(x) = &self._left {
            Display::fmt(x.as_ref(), f);
        }
        if let Some(x) = &self._right {
            Display::fmt(&x, f);
        }
        write!(f, "")
    }
}

impl<T: Debug> Display for BST<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(x) = &self._root {
            Display::fmt(x.as_ref(), f);
        }
        write!(f, "None")
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
    bst.insert(13);
    bst.insert(11);
    let node = bst.get_node(&13);
    // println!("node:{:#?}", node);
    // bst.enumerate();
    println!("node:{}", bst);
    // let datas = bst.post_order();
    // println!("datas: {:?}", datas);
}
