use std::{
    cell::RefCell,
    fmt::Debug,
    ops::{DerefMut, Not},
    rc::Rc,
};

use crate::common::{
    node::{BoxEntity, BoxNode, RcRefEntity, RcRefNode},
    pos::Pos,
};

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
}

impl<T: Clone + Ord + PartialEq + Debug> RBTree<T> {
    pub fn insert(&mut self, val: T) {
        println!("\nval={:?}", val);
        let node = _insert(&mut self._root, val);
        _fix_after_insert(node, &self._root);
    }

    pub fn delete(&mut self, val: &T) {
        if let Some(x) = self._root.as_mut() {
            x.borrow_mut().delete(val);
        }
    }
}

impl<T: Clone + Debug> RBTree<T> {
    fn _check(&self) -> bool {
        if let Some(x) = &self._root {
            let x_borrow = x.borrow();
            if x_borrow._color == Color::Red {
                return false;
            }
            let mut count = 0;
            x_borrow._count(&mut count);
            _check(&self._root.clone(), count, 0)
        } else {
            true
        }
    }
}

impl<T> RBNode<T> {
    pub fn new(val: T, color: Color, parent: Option<Rc<RefCell<RBNode<T>>>>) -> Self {
        Self {
            _left: None,
            _right: None,
            _parent: parent,
            _val: Some(val),
            _color: color,
        }
    }
    pub fn new_red(val: T) -> Self {
        Self::new(val, Color::Red, None)
    }
    pub fn new_black(val: T) -> Self {
        Self::new(val, Color::Black, None)
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
        //性质5
        if child._color == Color::Black {
            *count += 1;
        }
        true
    }

    //变色
    fn _change_color(&mut self, pos: Pos) {
        self._color = Color::Black;
        if let Some(x) = &self._parent {
            x.borrow_mut()._color = Color::Red;
            match pos {
                Pos::Left => {
                    if let Some(y) = &x.borrow_mut()._right {
                        y.borrow_mut()._color = Color::Black;
                    }
                }
                Pos::Right => {
                    if let Some(y) = &x.borrow_mut()._left {
                        y.borrow_mut()._color = Color::Black;
                    }
                }
            }
        }
        //子不需要变，因为本来就是红
    }
}

impl<T: Clone + Ord + Debug> RBNode<T> {
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

fn _check<T: Clone>(pnode: &Option<Rc<RefCell<RBNode<T>>>>, count: usize, mut tmp: usize) -> bool {
    if pnode.is_none() {
        return true;
    }
    let node = pnode.clone().unwrap().borrow().clone();
    if node._color == Color::Black {
        tmp += 1;
    }

    let parent = node._parent.clone();
    if let Some(x) = parent {
        if x.borrow()._color == Color::Red && node._color == Color::Red {
            return false;
        }
    }
    if pnode.is_none() && tmp != count {
        return false;
    }
    _check(&node._left, count, tmp) && _check(&node._right, count, tmp)
}

fn _get_node<T: Ord + Debug>(
    mut node: &Option<Rc<RefCell<RBNode<T>>>>,
    val: &Option<T>,
) -> Option<Rc<RefCell<RBNode<T>>>> {
    if node.is_none() {
        return None;
    }
    let mut node = node.clone();
    while node.is_some() {
        let tmp = node.clone().unwrap();
        let tmp = tmp.borrow();
        match tmp._val.cmp(val) {
            std::cmp::Ordering::Equal => {
                return node.clone();
            }
            std::cmp::Ordering::Less => {
                node = tmp._right.clone();
            }
            std::cmp::Ordering::Greater => {
                node = tmp._left.clone();
            }
        }
    }
    return None;
}

pub fn _insert<T: Ord + Clone>(
    this: &mut Option<Rc<RefCell<RBNode<T>>>>,
    val: T,
) -> Option<Rc<RefCell<RBNode<T>>>> {
    if this.is_none() {
        let root = Rc::new(RefCell::new(RBNode::new_black(val.clone())));
        *this = Some(root.clone());
        return Some(root);
    }
    let mut this = this.clone();
    while this.is_some() {
        let tmp = this.unwrap();
        let mut tb = tmp.borrow_mut();
        match tb._val.cmp(&Some(val.clone())) {
            std::cmp::Ordering::Less => {
                if tb._right.is_some() {
                    this = tb._right.clone();
                } else {
                    //self->right 设为new_node
                    let rc = Rc::new(RefCell::new(RBNode::new_red(val)));
                    tb._right = Some(rc.clone());
                    //new_node->parent设为self
                    rc.borrow_mut()._parent = Some(tmp.clone());
                    return Some(rc.clone());
                }
            }
            std::cmp::Ordering::Greater => {
                if tb._left.is_some() {
                    this = tb._left.clone();
                } else {
                    //self->left 设为new_node
                    let rc = Rc::new(RefCell::new(RBNode::new_red(val)));
                    tb._left = Some(rc.clone());
                    //new_node->parent设为self
                    rc.borrow_mut()._parent = Some(tmp.clone());
                    return Some(rc.clone());
                }
            }
            std::cmp::Ordering::Equal => break,
        }
    }
    None
}

/// 插入后处理旋转变色
fn _fix_after_insert<T: PartialEq + Debug>(
    mut this: Option<Rc<RefCell<RBNode<T>>>>,
    root: &Option<Rc<RefCell<RBNode<T>>>>,
) {
    while this.is_some() && this != *root {
        let x = this.clone().unwrap();
        let p = x.borrow()._parent.clone().unwrap();
        if p.borrow()._color == Color::Black {
            break;
        }
        let grandparent = p.borrow()._parent.clone();
        let uncle;
        let pos = if let Some(gp) = &grandparent {
            if gp.borrow()._left == Some(p.clone()) {
                uncle = gp.borrow()._right.clone();
                Pos::Left
            } else {
                uncle = gp.borrow()._left.clone();
                Pos::Right
            }
        } else {
            uncle = None;
            Pos::Right
        };
        println!("uncle={:?}", uncle);
        if let Some(u) = uncle {
            let mut ub = u.borrow_mut();
            match ub._color {
                Color::Red => {
                    p.borrow_mut()._color = Color::Black;
                    ub._color = Color::Black;
                    if let Some(gp) = &grandparent {
                        gp.borrow_mut()._color = Color::Red;
                    }
                    this = grandparent.clone();
                }
                Color::Black => {
                    match pos {
                        Pos::Left => {
                            if this == p.borrow()._right {
                                this = Some(p.clone());
                                //左旋this
                            }
                            p.borrow_mut()._color = Color::Black;
                            if let Some(gp) = &grandparent {
                                gp.borrow_mut()._color = Color::Red;
                                //右旋gp
                            }
                        }
                        Pos::Right => {
                            if this == p.borrow()._left {
                                this = Some(p.clone());
                                //右旋this
                            }
                            p.borrow_mut()._color = Color::Black;
                            if let Some(gp) = &grandparent {
                                gp.borrow_mut()._color = Color::Red;
                                //左旋gp
                            }
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
    if let Some(r) = root.as_ref() {
        r.borrow_mut()._color = Color::Black;
    }
}

impl<T: Clone + PartialEq + Debug> RBNode<T> {
    //执行旋转或变色
    //pos: 当前为左子树就传left，右子树就传right
    fn _operate(&mut self, root: &mut Self, pos: Pos) {
        if self._parent.is_none() {
            return;
        }

        #[derive(Debug)]
        enum Stat {
            BL,
            BR,
            R,
        }
        let judge_uncle_stat = |color: Color| -> Stat {
            match color {
                Color::Black => match pos {
                    Pos::Left => Stat::BL,
                    Pos::Right => Stat::BR,
                },
                Color::Red => Stat::R,
            }
        };
        /// 父红，叔黑，且当前节点是右子树，以父节点左旋
        /// 父红，叔黑，且当前节点是左子树，以父节点右旋
        /// 父节点为红，叔叔也为红时，变色
        let stat = if let Some(p) = &self._parent {
            // 父黑
            if p.borrow()._color == Color::Black {
                return;
            }
            // println!("parent={:?}", p);
            if let Some(gp) = &p.borrow()._parent {
                let gpb = gp.borrow();
                // println!(
                //     "gpb: _left={:?}, _right={:?}, val={:?}, color={:?}",
                //     gpb._left,
                //     gpb._right,
                //     p.borrow()._val,
                //     p.borrow()._color
                // );
                let pc = Some(p.clone());
                if gpb._right == pc {
                    println!("is right");
                    if let Some(u) = &gpb._left {
                        judge_uncle_stat(u.borrow()._color)
                    } else {
                        judge_uncle_stat(Color::Black)
                    }
                } else if gpb._left == pc {
                    println!("is left");
                    if let Some(u) = &gpb._right {
                        judge_uncle_stat(u.borrow()._color)
                    } else {
                        judge_uncle_stat(Color::Black)
                    }
                } else {
                    println!("error");
                    return;
                }
            } else {
                return;
            }
        } else {
            return;
        };

        println!("stat={:?}\n", stat);
        match stat {
            Stat::BL => self._rotatel(root),
            Stat::BR => self._rotater(root),
            Stat::R => self._change_color(pos),
        }
    }
    //左旋，self为4点
    ///////////////////////////////////////////////
    ///      2              左旋              4
    /// 1        4         -->       2             5
    ///        3   5      <--     1  3
    ///                     右旋
    //////////////////////////////////////////////
    fn _rotatel(&mut self, root: &mut Self) {
        let mut r = self._right.clone();
        let mut gp = self._parent.clone();
        let this = Rc::new(RefCell::new(self.clone()));
        if let Some(x) = &r {
            let rl = x.borrow()._left.clone();
            self._right = rl.clone();
            if let Some(y) = &rl {
                y.borrow_mut()._parent = Some(this.clone());
            }
            x.borrow_mut()._left = Some(this.clone());

            self._parent = Some(x.clone());
            x.borrow_mut()._parent = gp.clone();
            if gp.is_none() {
                *root = x.borrow().clone();
            }
        }
        if gp.is_some() {
            let gpu = gp.clone().unwrap();
            let pos = if gpu.borrow()._left == Some(this.clone()) {
                Pos::Left
            } else {
                Pos::Right
            };
            match pos {
                Pos::Left => gpu.borrow_mut()._left = r.clone(),
                Pos::Right => gpu.borrow_mut()._right = r.clone(),
            }
        }
    }
    fn _rotater(&mut self, root: &mut Self) {
        let mut l = self._left.clone();
        let mut gp = self._parent.clone();
        let this = Rc::new(RefCell::new(self.clone()));
        if let Some(x) = &l {
            let lr = x.borrow()._right.clone();
            self._right = lr.clone();
            if let Some(y) = &lr {
                y.borrow_mut()._parent = Some(this.clone());
            }
            x.borrow_mut()._right = Some(this.clone());

            self._parent = Some(x.clone());
            x.borrow_mut()._parent = gp.clone();
            if gp.is_none() {
                *root = x.borrow().clone();
            }
        }
        if gp.is_some() {
            let gpu = gp.unwrap();
            if gpu.borrow()._left == Some(this.clone()) {
                gpu.borrow_mut()._left = l.clone();
            } else {
                gpu.borrow_mut()._right = l.clone();
            }
        }
    }
    fn _rotate_left(&mut self) {
        let this = Rc::new(RefCell::new(self.clone()));
        let p_clone = self._parent.clone();
        let mut r_left = self._left.clone();
        if let Some(p) = &p_clone {
            let clonep = p.borrow().clone();
            if let Some(pp) = &clonep._parent {
                //设置4的父级为2的父级
                this.borrow_mut()._parent = Some(pp.clone());
                //设置2的父级的子级为4
                let is_left = pp.borrow()._left == Some(p.clone());
                let tmp = Some(this.clone());
                if is_left {
                    pp.borrow_mut()._left = tmp;
                } else {
                    pp.borrow_mut()._right = tmp;
                }
            }
            //设置2的父级为4
            p.borrow_mut()._parent = Some(this.clone());
            //设置4的子级为2
            let p_opt = Some(Rc::new(RefCell::new(p.borrow().clone())));
            this.borrow_mut()._left = p_opt.clone();
            //设置3的父级为2
            if let Some(x) = &r_left {
                x.borrow_mut()._parent = p_opt;
            }
            //设置2的右侧为3
            p.borrow_mut()._right = r_left;
        }
    }
    //右旋，self为2点
    ///////////////////////////////////////////////
    ///      2              左旋              4
    /// 1        4         -->       2             5
    ///        3   5      <--     1  3
    ///                     右旋
    //////////////////////////////////////////////
    fn _rotate_right(&mut self) {
        let this = Rc::new(RefCell::new(self.clone()));
        let p_clone = self._parent.clone();
        let mut r_right = self._right.clone();
        if let Some(p) = p_clone.as_ref() {
            if let Some(pp) = p.borrow()._parent.as_ref() {
                //设置4的父级的子级为2
                let is_left = pp.borrow()._left == Some(p.clone());
                let tmp = Some(this.clone());
                if is_left {
                    pp.borrow_mut()._left = tmp;
                } else {
                    pp.borrow_mut()._right = tmp;
                }
                //设置2的父级为4的父级
                this.borrow_mut()._parent = Some(pp.clone());
            }
            //设置4的父级为2
            p.borrow_mut()._parent = Some(this.clone());
            let p_opt = Some(Rc::new(RefCell::new(p.borrow().clone())));
            //设置2的子级为4
            this.borrow_mut()._right = p_opt.clone();
            //设置3的父级为4
            if let Some(x) = &r_right {
                x.borrow_mut()._parent = p_opt;
            }
            //设置4的左侧为3
            p.borrow_mut()._left = r_right;
        };
    }
}

impl<T: Ord + Clone> RcRefEntity<T> for RBTree<T> {
    type U = RBNode<T>;

    fn get_node(&self, t: &T) -> Option<Rc<RefCell<Self::U>>> {
        if let Some(x) = &self._root {
            return x.borrow().get_node(&Some(t));
        }
        None
    }
}

impl<T: Ord + Clone> RcRefNode<T> for RBNode<T> {
    type U = Self;

    fn get_node(&self, t: &Option<&T>) -> Option<Rc<RefCell<Self::U>>> {
        match t.cmp(&self._val.as_ref()) {
            std::cmp::Ordering::Equal => {
                // if let Some(x) = &self._parent {
                //     let tmp = Some(Rc::new(RefCell::new(self.clone())));
                //     if x.borrow()._left == tmp {
                //         return x.borrow()._left.clone();
                //     } else {
                //         return None;
                //     }
                // }
                Some(Rc::new(RefCell::new((*self).clone())))
            }
            std::cmp::Ordering::Less => {
                if let Some(x) = &self._left {
                    x.borrow().get_node(t)
                } else {
                    None
                }
            }
            std::cmp::Ordering::Greater => {
                if let Some(x) = &self._right {
                    x.borrow().get_node(t)
                } else {
                    None
                }
            }
        }
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

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::Red,
            Color::Red => Color::Black,
        }
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
    // let mut rbt = RBTree::new(10);
    // rbt.insert(11);
    // // rbt.insert(12);
    // rbt.insert(9);
    // rbt.insert(8);
    // rbt.insert(7);
    // println!();
    // // rbt.insert(10);
    // println!("rbt={:?}, check:{:?}", rbt, rbt._check());
    let mut rbt = RBTree::new(50);
    rbt.insert(29);
    // rbt.insert(12);
    rbt.insert(77);
    rbt.insert(10);
    // rbt.insert(30);
    // rbt.insert(55);
    // rbt.insert(80);
    // rbt.insert(18);
    // rbt.insert(51);
    // rbt.insert(66);
    // rbt.insert(90);
    println!();
    // rbt.insert(10);
    println!("rbt={:?}, check:{:?}", rbt, rbt._check());
}
