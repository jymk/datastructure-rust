use std::{
    cell::RefCell,
    fmt::Debug,
    ops::{DerefMut, Not},
    rc::Rc,
};

use crate::common::pos::Pos;

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

impl<T: Clone + Ord + Debug> RBTree<T> {
    fn get_node(&self, t: &T) -> RBInnerNode<T> {
        _get_node(&self._root, &Some(t.clone()))
    }
}

impl<T: Clone + Ord + PartialEq + Debug> RBTree<T> {
    pub fn insert(&mut self, val: T) {
        // println!("\nval={:?}", val);
        let node = _insert(&mut self._root, val);
        _fix_after_insert(node, &mut self._root);
    }

    pub fn delete(&mut self, val: &T) {
        _delete(&mut self._root, val);
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
    pub fn new(val: T, color: Color, parent: RBInnerNode<T>) -> Self {
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

fn _check<T: Clone>(pnode: &RBInnerNode<T>, count: usize, mut tmp: usize) -> bool {
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

fn _get_node<T: Ord + Debug>(root: &RBInnerNode<T>, val: &Option<T>) -> RBInnerNode<T> {
    if root.is_none() {
        return None;
    }
    let mut cur = root.clone();
    while cur.is_some() {
        let tmp = cur.clone().unwrap();
        let tmp = tmp.borrow();
        match tmp._val.cmp(val) {
            std::cmp::Ordering::Equal => {
                return cur.clone();
            }
            std::cmp::Ordering::Less => {
                cur = tmp._right.clone();
            }
            std::cmp::Ordering::Greater => {
                cur = tmp._left.clone();
            }
        }
    }
    return None;
}

/// 参考java
/// 此处对我的难点是当去掉一个中间的节点之后，是用它的左节点还是右节点作为新的中间节点呢
fn _delete<T: Ord + Clone + PartialEq + Debug>(root: &mut RBInnerNode<T>, val: &T) {
    if root.is_none() {
        return;
    }

    let mut node = _get_node(&root, &Some(val.clone()));
    if node.is_none() {
        return;
    }
    let noder = node.clone().unwrap();
    if noder.borrow()._left.is_none() && noder.borrow()._right.is_none() {
        let s = _successor(&node);
        let sr = s.clone().unwrap();
        let mut sbm = sr.borrow_mut();
        noder.borrow_mut()._val = sbm._val.clone();
        node = s.clone();
    }
    let noder = node.clone().unwrap();
    let mut nodebm = noder.borrow_mut();
    let replacement = if nodebm._left.is_some() {
        nodebm._left.clone()
    } else {
        nodebm._right.clone()
    };

    if replacement.is_some() {
        let replace = replacement.clone().unwrap();
        replace.borrow_mut()._parent = nodebm._parent.clone();
        if nodebm._parent.is_none() {
            *root = replacement.clone();
        } else {
            let parent = nodebm._parent.clone().unwrap();
            let mut pbm = parent.borrow_mut();
            if node == pbm._left {
                pbm._left = replacement.clone();
            } else {
                pbm._right = replacement.clone();
            }
        }

        nodebm._left = None;
        nodebm._right = None;
        nodebm._parent = None;

        if nodebm._color == Color::Black {
            _fix_after_del(&replacement, root);
        }
    } else if nodebm._parent.is_none() {
        *root = None;
    } else {
        if nodebm._color == Color::Black {
            _fix_after_del(&node, root);
        }

        if nodebm._parent.is_some() {
            let parent = nodebm._parent.clone().unwrap();
            let mut parentbm = parent.borrow_mut();
            if node == parentbm._left {
                parentbm._left = None;
            } else if node == parentbm._right {
                parentbm._right = None;
            }
            nodebm._parent = None;
        }
    }
}

//寻找比t大的最小值
fn _successor<T: PartialEq>(t: &RBInnerNode<T>) -> RBInnerNode<T> {
    if t.is_none() {
        return None;
    }
    let tb = t.clone().unwrap();
    let tb = tb.borrow();
    if tb._right.is_some() {
        let mut p = tb._right.clone();
        let pb = p.clone().unwrap();
        let pb = pb.borrow();
        while pb._right.is_some() {
            p = pb._left.clone();
        }
        p.clone()
    } else {
        let mut p = tb._parent.clone();
        let mut ch = t.clone();
        while p.is_some() {
            let pb = p.clone().unwrap();
            let pb = pb.borrow();
            if ch != pb._right {
                break;
            }
            ch = p.clone();
            p = pb._parent.clone();
        }
        p.clone()
    }
}

pub fn _insert<T: Ord + Clone>(this: &mut RBInnerNode<T>, val: T) -> RBInnerNode<T> {
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
/// 								red -> parent设黑、uncle设黑、gp设红、x设为gp
///	    parent为左    y uncle
/// 								black->(x若为p的右，设为parent并左旋)、parent设黑、gp设红、右旋gp
/// 新节点x(不为空、不为root、且父级为红)																	root设黑
/// 								red -> parent设黑、uncle设黑、gp设红、x设为gp
/// 		parent为右    y uncle
/// 								black->(x若为p的左，设为parent并右旋)、parent设黑、gp设红、左旋gp
fn _fix_after_insert<T: PartialEq + Debug>(mut this: RBInnerNode<T>, root: &mut RBInnerNode<T>) {
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
        // println!("uncle={:?}", uncle);
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
                                _rotate_left(&this, root);
                            }
                            p.borrow_mut()._color = Color::Black;
                            if let Some(gp) = &grandparent {
                                gp.borrow_mut()._color = Color::Red;
                                //右旋gp
                                _rotate_right(&Some(gp.clone()), root);
                            }
                        }
                        Pos::Right => {
                            if this == p.borrow()._left {
                                this = Some(p.clone());
                                //右旋this
                                _rotate_right(&this, root);
                            }
                            p.borrow_mut()._color = Color::Black;
                            if let Some(gp) = &grandparent {
                                gp.borrow_mut()._color = Color::Red;
                                //左旋gp
                                _rotate_left(&Some(gp.clone()), root);
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

fn _fix_after_del<T: PartialEq + Clone + Debug>(x: &RBInnerNode<T>, root: &mut RBInnerNode<T>) {
    let mut x = x.clone();
    while x != *root && _color_of(&x) == Color::Black {
        if x == _left_of(&_parent_of(&x)) {
            let mut sib = _right_of(&_parent_of(&x));

            if _color_of(&sib) == Color::Red {
                _set_color(&sib, Color::Black);
                _set_color(&_parent_of(&x), Color::Red);
                _rotate_left(&_parent_of(&x), root);
                sib = _right_of(&_parent_of(&x))
            }

            if _color_of(&_left_of(&sib)) == Color::Black
                && _color_of(&_right_of(&sib)) == Color::Black
            {
                _set_color(&sib, Color::Red);
                x = _parent_of(&x);
            } else {
                if _color_of(&_right_of(&sib)) == Color::Black {
                    _set_color(&_left_of(&sib), Color::Black);
                    _set_color(&sib, Color::Red);
                    _rotate_right(&sib, root);
                    sib = _right_of(&_parent_of(&x));
                }

                _set_color(&sib, _color_of(&_parent_of(&x)));
                _set_color(&_parent_of(&x), Color::Black);
                _set_color(&_right_of(&sib), Color::Black);
                _rotate_left(&_parent_of(&x), root);
                x = root.clone();
            }
        } else {
            let mut sib = _left_of(&_parent_of(&x));

            if _color_of(&sib) == Color::Red {
                _set_color(&sib, Color::Black);
                _set_color(&_parent_of(&x), Color::Red);
                _rotate_right(&_parent_of(&x), root);
                sib = _left_of(&_parent_of(&x))
            }

            if _color_of(&_right_of(&sib)) == Color::Black
                && _color_of(&_left_of(&sib)) == Color::Black
            {
                _set_color(&sib, Color::Red);
                x = _parent_of(&x);
            } else {
                if _color_of(&_left_of(&sib)) == Color::Black {
                    _set_color(&_right_of(&sib), Color::Black);
                    _set_color(&sib, Color::Red);
                    _rotate_left(&sib, root);
                    sib = _left_of(&_parent_of(&x));
                }

                _set_color(&sib, _color_of(&_parent_of(&x)));
                _set_color(&_parent_of(&x), Color::Black);
                _set_color(&_left_of(&sib), Color::Black);
                _rotate_right(&_parent_of(&x), root);
                x = root.clone();
            }
        }
    }
    _set_color(&x, Color::Black);
}

fn _set_color<T>(x: &RBInnerNode<T>, c: Color) {
    if let Some(y) = &x {
        y.borrow_mut()._color = c;
    }
}

fn _parent_of<T: Clone>(x: &RBInnerNode<T>) -> RBInnerNode<T> {
    if let Some(y) = &x {
        y.borrow()._parent.clone()
    } else {
        None
    }
}

fn _left_of<T: Clone>(x: &RBInnerNode<T>) -> RBInnerNode<T> {
    if let Some(y) = &x {
        y.borrow()._left.clone()
    } else {
        None
    }
}

fn _right_of<T: Clone>(x: &RBInnerNode<T>) -> RBInnerNode<T> {
    if let Some(y) = &x {
        y.borrow()._right.clone()
    } else {
        None
    }
}

fn _color_of<T: Clone>(x: &RBInnerNode<T>) -> Color {
    if x.is_none() {
        return Color::Black;
    }
    let y = x.clone().unwrap();
    let yb = y.borrow().clone();
    yb._color
}

///////////////////////////////////////////////
///      2              左旋              4
/// 1        4         -->       2             5
///        3   5      <--     1  3
///                     右旋
//////////////////////////////////////////////
//参考java->TreeMap->rotateLeft
//这里自己实现应该也没什么问题，只是之前陷入了传参为RBNode而非Rc RefCell RBNode，会使RBNode并非一个对象的问题
fn _rotate_left<T: PartialEq>(this: &RBInnerNode<T>, root: &mut RBInnerNode<T>) {
    if this.is_none() {
        return;
    }
    let thisu = this.clone().unwrap();
    let mut thisbm = thisu.borrow_mut();
    //right
    let r = thisbm._right.clone().unwrap();
    let mut rbm = r.borrow_mut();
    //this->right = right->left
    thisbm._right = rbm._left.clone();

    //right->left->parent = this
    if rbm._left.is_some() {
        let rbml = rbm._left.clone().unwrap();
        rbml.borrow_mut()._parent = this.clone();
    }
    //right->parent = this->parent
    rbm._parent = thisbm._parent.clone();
    //root = right
    if thisbm._parent.is_none() {
        *root = Some(r.clone());
    } else {
        let parent = thisbm._parent.clone().unwrap();
        let mut pbm = parent.borrow_mut();
        if pbm._left == *this {
            //this->parent->left = right
            pbm._left = Some(r.clone());
        } else {
            //this->parent->right = right
            pbm._right = Some(r.clone());
        }
    }
    //right->left = this
    rbm._left = this.clone();
    //this->parent = right;
    thisbm._parent = Some(r.clone());
}

fn _rotate_right<T: PartialEq>(this: &RBInnerNode<T>, root: &mut RBInnerNode<T>) {
    if this.is_none() {
        return;
    }
    let thisu = this.clone().unwrap();
    let mut thisbm = thisu.borrow_mut();
    //left
    let l = thisbm._left.clone().unwrap();
    let mut lbm = l.borrow_mut();
    //this->left = right->right
    thisbm._left = lbm._right.clone();

    //left->right->parent = this
    if lbm._right.is_some() {
        let lbml = lbm._right.clone().unwrap();
        lbml.borrow_mut()._parent = this.clone();
    }
    //left->parent = this->parent
    lbm._parent = thisbm._parent.clone();
    //root = left
    if thisbm._parent.is_none() {
        *root = Some(l.clone());
    } else {
        let parent = thisbm._parent.clone().unwrap();
        let mut pbm = parent.borrow_mut();
        if pbm._right == *this {
            //this->parent->right = left
            pbm._right = Some(l.clone());
        } else {
            //this->parent->left = left
            pbm._left = Some(l.clone());
        }
    }
    //left->right = this
    lbm._right = this.clone();
    //this->parent = left;
    thisbm._parent = Some(l.clone());
}

//此处代码全部未用到
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
    rbt.insert(30);
    rbt.insert(55);
    rbt.insert(80);
    rbt.insert(18);
    rbt.insert(51);
    rbt.insert(66);
    rbt.insert(90);
    rbt.delete(&29);
    println!();
    // rbt.insert(10);
    println!("rbt={:?}", rbt);
    println!("check:{:?}", rbt._check());
}
