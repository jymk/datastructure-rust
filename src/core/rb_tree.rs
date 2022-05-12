use std::{cell::RefCell, fmt::Debug, ops::DerefMut, rc::Rc};

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
#[derive(Clone, PartialEq, Eq)]
pub struct RBTree<T> {
    _root: RBInnerNode<T>,
}

type RBInnerNode<T> = Option<Rc<RefCell<RBNode<T>>>>;

#[derive(Clone, PartialEq, Eq)]
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
            _root: Some(Rc::new(RefCell::new(RBNode::new_black(val)))),
        }
    }
    fn _check(&self) -> bool {
        if let Some(x) = &self._root {
            if x.borrow()._color == Color::Red {
                return false;
            }
            let mut count = 0;
            x.borrow()._count(&mut count);
            x.borrow()._check(count)
        } else {
            true
        }
    }
}

impl<T: Clone + Ord> RBTree<T> {
    pub fn insert(&mut self, val: T) {
        if let Some(x) = self._root.as_mut() {
            x.borrow_mut().insert(val);
        } else {
            self._root = Some(Rc::new(RefCell::new(RBNode::new_black(val))));
        }
    }
    pub fn delete(&mut self, val: &T) {
        if let Some(x) = self._root.as_mut() {
            x.borrow_mut().delete(val);
        }
    }
}

impl<T: Clone> RBTree<T> {
    //左旋，node为4点
    ///////////////////////////////////////////////
    ///      2              左旋              4
    /// 1        4         -->       2             5
    ///        3   5      <--     1  3
    ///                     右旋
    //////////////////////////////////////////////
    fn _rotate_left(&mut self, node: &mut RBNode<T>) {
        if let Some(x) = self._root.as_mut() {
            node._rotate_left(x.borrow_mut().deref_mut())
        }
    }
    //右旋，node为2点
    ///////////////////////////////////////////////
    ///      2              左旋              4
    /// 1        4         -->       2             5
    ///        3   5      <--     1  3
    ///                     右旋
    //////////////////////////////////////////////
    fn _rotate_right(&mut self, node: &mut RBNode<T>) {
        if let Some(x) = self._root.as_mut() {
            node._rotate_right(x.borrow_mut().deref_mut())
        }
    }
}

impl<T> RBNode<T> {
    pub fn new(val: T, color: Color) -> Self {
        Self {
            _left: None,
            _right: None,
            _parent: None,
            _val: Some(val),
            _color: color,
        }
    }
    pub fn new_red(val: T) -> Self {
        Self::new(val, Color::Red)
    }
    pub fn new_black(val: T) -> Self {
        Self::new(val, Color::Black)
    }
    //统计最左侧链的黑色总数
    fn _count(&self, count: &mut usize) {
        if let Some(x) = &self._left {
            if x.borrow()._color == Color::Black {
                *count += 1;
            }
            x.borrow()._count(count)
        }
    }
    //判断节点是否符合要求
    fn _judge_node(&self, child: &Self, count: &mut usize) -> bool {
        //性质4
        if self._color == Color::Red && child._color == Color::Red {
            return false;
        }
        if child._color == Color::Black {
            *count += 1;
        }
        true
    }
    fn _check(&self, count: usize) -> bool {
        let mut tmp = 0;
        if let Some(x) = &self._left {
            if !self._judge_node(&x.borrow(), &mut tmp) {
                return false;
            }
            if !x.borrow()._check(count) {
                return false;
            }
        }
        if tmp != count {
            return false;
        }
        tmp = 0;
        if let Some(x) = &self._right {
            if !self._judge_node(&x.borrow(), &mut tmp) {
                return false;
            }
            if !x.borrow()._check(count) {
                return false;
            }
        }
        if tmp != count {
            return false;
        }
        true
    }
}
impl<T: Clone + Ord> RBNode<T> {
    pub fn insert(&mut self, val: T) {
        if let Some(x) = self._val.as_ref() {
            match val.cmp(&x) {
                std::cmp::Ordering::Less => {
                    if let Some(y) = &self._right {
                        y.borrow_mut().insert(val);
                    } else {
                        self._right = Some(Rc::new(RefCell::new(RBNode::new_red(val))));
                    }
                }
                std::cmp::Ordering::Greater => {
                    if let Some(y) = &self._left {
                        y.borrow_mut().insert(val);
                    } else {
                        self._left = Some(Rc::new(RefCell::new(RBNode::new_red(val))));
                    }
                }
                std::cmp::Ordering::Equal => return,
            }
        } else {
            self._val = Some(val);
        }
    }
    pub fn delete(&mut self, val: &T) {
        if let Some(x) = self._val.as_ref() {
            match val.cmp(x) {
                std::cmp::Ordering::Equal => {
                    if let Some(c) = &self._left {
                        c.borrow_mut()._parent = self._parent.clone();
                    }
                    if let Some(p) = &self._parent {
                        p.borrow_mut()._left = self._left.clone();
                    }
                }
                std::cmp::Ordering::Less => {
                    if let Some(y) = &self._left {
                        y.borrow_mut().delete(val);
                    }
                }
                std::cmp::Ordering::Greater => {
                    if let Some(y) = &self._right {
                        y.borrow_mut().delete(val);
                    }
                }
            }
        }
    }
}
impl<T: Clone> RBNode<T> {
    //左旋，self为4点
    ///////////////////////////////////////////////
    ///      2              左旋              4
    /// 1        4         -->       2             5
    ///        3   5      <--     1  3
    ///                     右旋
    //////////////////////////////////////////////
    pub(crate) fn _rotate_left(&mut self, root: &mut RBNode<T>) {
        let p_clone = self._parent.clone();
        let mut r_left = self._left.clone();
        if let Some(p) = p_clone.as_ref() {
            if let Some(pp) = p.borrow()._parent.as_ref() {
                //设置4的父级为2的父级
                self._parent = Some(pp.clone());
            }
            self._left = Some(p.clone());
            p.borrow_mut()._parent = Some(Rc::new(RefCell::new(self.clone())));
            p.borrow_mut()._right = r_left;
        };
    }
    //右旋，self为2点
    ///////////////////////////////////////////////
    ///      2              左旋              4
    /// 1        4         -->       2             5
    ///        3   5      <--     1  3
    ///                     右旋
    //////////////////////////////////////////////
    pub(crate) fn _rotate_right(&mut self, root: &mut RBNode<T>) {
        let p_clone = self._parent.clone();
        let mut r_right = self._right.clone();
        if let Some(p) = p_clone.as_ref() {
            if let Some(pp) = p.borrow()._parent.as_ref() {
                //设置2的父级为4的父级
                self._parent = Some(pp.clone());
            }
            self._right = Some(p.clone());
            p.borrow_mut()._parent = Some(Rc::new(RefCell::new(self.clone())));
            p.borrow_mut()._left = r_right;
        };
    }
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

impl<T: Debug> Debug for RBNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "val: {:?}, color: {:?} => ", self._val, self._color)?;
        if let Some(x) = &self._left {
            Debug::fmt(&x.borrow(), f);
        }
        if let Some(x) = &self._right {
            Debug::fmt(&x.borrow(), f);
        }
        write!(f, "")
    }
}

impl<T: Debug> Debug for RBTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(x) = &self._root {
            Debug::fmt(&x.borrow(), f);
        }
        write!(f, "None")
    }
}

#[test]
fn test() {
    let mut rbt = RBTree::new(10);
    rbt.insert(11);
    rbt.insert(12);
    rbt.insert(9);
    rbt.insert(10);
    println!("rbt={:?}", rbt);
}
