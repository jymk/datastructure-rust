use crate::errs::OUT_OF_RANGE;
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut, Index, IndexMut},
    rc::Rc,
};

/// 双向链表
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DList<T> {
    _head: InnerNode<T>,
    _len: usize,
}

//rc起到指针的作用，为了使prev引的对象与上上一个的next的是同一个对象
type InnerNode<T> = Option<Rc<RefCell<DNode<T>>>>;

#[derive(Clone, PartialEq, Eq)]
pub struct DNode<T> {
    _val: T,
    _next: InnerNode<T>,
    _prev: InnerNode<T>,
}
impl<T: Clone> DNode<T> {
    pub fn new(val: T) -> Self {
        DNode {
            _val: val,
            _next: None,
            _prev: None,
        }
    }
    pub fn get_value(&self) -> &T {
        &self._val
    }

    //下一个节点
    pub fn next(&self) -> Option<&Rc<RefCell<Self>>> {
        match &self._next {
            Some(v) => Some(v),
            None => None,
        }
    }
    //当前可变值
    pub fn get_mut(&mut self) -> &mut T {
        &mut self._val
    }
}

impl<T: Clone + Debug> DList<T> {
    pub fn new() -> Self {
        DList {
            _head: None,
            _len: usize::default(),
        }
    }
    //new并在尾部增加一个值
    // pub fn new_with_val(val: T) -> Self {
    //     let mut tmp = Self::new();
    //     tmp.add(val);
    //     tmp
    // }
    //在尾部增加一个值
    // pub fn add(&mut self, val: T) {
    //     self.add_at_tail(val);
    // }
    //获取index下标处节点
    pub fn get_node(&self, index: usize) -> Option<Rc<RefCell<DNode<T>>>> {
        let mut i = 0;
        let mut head = self._head.as_ref();
        let mut res = None;
        if let Some(h) = head {
            let mut cur = Some(h);
            while let Some(x) = &h.borrow_mut()._next {
                if i == index {
                    res = Some(x.clone());
                    break;
                }
                cur = x.borrow_mut()._next.as_ref();
                i += 1;
            }
        }
        res
    }
    // 获取index下标处不可变值
    pub fn get(&self, index: usize) -> Option<T> {
        if let Some(x) = &self.get_node(index) {
            let val = x.borrow()._val.clone();
            Some(val)
        } else {
            None
        }
    }

    //头插
    pub fn add_at_head(&mut self, val: T) {
        let mut new_node = DNode::new(val);
        if let Some(ref mut h) = self._head {
            let head = self._head.take().unwrap();
            new_node._next = Some(head.clone());
            let rc = Rc::new(RefCell::new(new_node));
            head.borrow_mut()._prev = Some(rc.clone());
            self._head = Some(rc)
        } else {
            self._head = Some(Rc::new(RefCell::new(new_node)));
        }
        self._len += 1;
    }

    //尾插
    pub fn add_at_tail(&mut self, val: T) {
        let len = self._len;
        let mut new_node = DNode::new(val.clone());
        if len == 0 {
            self.add_at_head(val);
            return;
        }
        let mod_node = self.get_node(len - 1);
        if let Some(x) = mod_node {
            new_node._prev = Some(x.clone());
            let rc = Rc::new(RefCell::new(new_node));
            x.borrow_mut()._next = Some(rc.clone());
            self._len += 1;
        }
    }

    // //下标插
    // pub fn add_at_index(&mut self, index: usize, val: T) {
    //     let len = self._len;
    //     if index > len {
    //         // panic!("out of range");
    //         return;
    //     }
    //     let mut new_node = Some(Rc::new(RefCell::new(DNode {
    //         _val: val.clone(),
    //         _next: None,
    //         _prev: None,
    //     })));
    //     //长度为0
    //     if len == 0 {
    //         self._head = new_node;
    //         self._len += 1;
    //         return;
    //     }
    //     let mut cur = self._head.as_ref();
    //     //下标为0
    //     if index == 0 {
    //         self.add_at_head(val);
    //         return;
    //     }
    //     let mut i = 0;
    //     while let Some(x) = cur {
    //         if i == index - 1 {
    //             let tmp = new_node.as_ref();
    //             if let Some(n) = tmp {
    //                 n.borrow_mut()._next = x.borrow()._next.clone();
    //             }
    //             x.borrow_mut()._next = new_node;
    //             self._len += 1;
    //             break;
    //         }
    //         cur = x.borrow().next();
    //         i += 1;
    //     }
    // }

    // //删头
    // pub fn delete_head(&mut self) {
    //     if let Some(x) = self._head.as_mut() {
    //         self._head = x.borrow()._next.clone();
    //         self._len -= 1;
    //     }
    // }
    // //删下标
    // pub fn delete_at_index(&mut self, index: usize) {
    //     let len = self._len;
    //     let mut cur = self._head.as_ref();
    //     //下标为0
    //     if len > 0 && index == 0 {
    //         self.delete_head();
    //         return;
    //     }
    //     let mut i = 0;
    //     while let Some(x) = cur {
    //         if i == index - 1 {
    //             let mid = x.borrow()._next.as_ref();
    //             if let Some(m) = mid {
    //                 let mut right = m.borrow()._next.as_ref();
    //                 if let Some(r) = right {
    //                     x.borrow_mut()._next = Some(r.clone());
    //                     self._len -= 1;
    //                 }
    //             }
    //             break;
    //         }
    //         cur = x.borrow().next();
    //         i += 1;
    //     }
    // }

    //长度
    pub fn len(&self) -> usize {
        self._len
    }

    //头节点
    pub fn next(&self) -> Option<&Rc<RefCell<DNode<T>>>> {
        match &self._head {
            Some(v) => Some(v),
            None => None,
        }
    }
    //清空
    pub fn clear(&mut self) {
        self._head = None;
        self._len = 0;
    }
}

/// 不实现DNode的打印方法，打印会循环引用而爆栈
impl<T: Debug> Display for DList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self._head, f)
    }
}

impl<T: Display> Display for DNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => ", self._val)?;
        if let Some(node) = &self._next {
            return std::fmt::Display::fmt(&node.borrow(), f);
        }
        write!(f, "")
    }
}
impl<T: Debug> Debug for DNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} => ", self._val)?;
        if let Some(node) = &self._next {
            return Debug::fmt(&node.borrow(), f);
        }
        write!(f, "")
    }
}

#[test]
fn test() {
    let mut dl = DList::<i32>::new();
    dl.add_at_head(3);
    dl.add_at_head(4);
    dl.add_at_tail(5);
    dl.add_at_tail(7);
    dl.add_at_tail(6);
    println!("dl:{}", dl);
}
