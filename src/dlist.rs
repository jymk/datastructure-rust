use crate::errs::OUT_OF_RANGE;
use std::{
    cell::RefCell,
    fmt::Debug,
    ops::{Deref, DerefMut, Index, IndexMut},
    rc::Rc,
};

/// 双向链表
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DList<T> {
    _head: InnerNode<T>,
    _len: usize,
}

type InnerNode<T> = Option<Rc<RefCell<DNode<T>>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DNode<T> {
    _val: T,
    _next: InnerNode<T>,
    _prev: InnerNode<T>,
}
impl<T: Clone> DNode<T> {
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
    //获取index下标处可变节点
    // pub fn get_node(&self, index: usize) -> Option<&Rc<RefCell<DNode<T>>>> {
    //     let mut i = 0;
    //     let mut cur = self._head.as_ref();
    //     let mut res = None;
    //     while let Some(x) = cur {
    //         if i == index {
    //             res = Some(x);
    //             break;
    //         }
    //         cur = x.borrow().next();
    //         i += 1;
    //     }
    //     res
    // }
    // //获取index下标处可变值
    // pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
    //     let mut i = 0;
    //     let head = &mut self._head;
    //     let mut cur = head.as_mut();
    //     let mut res = None;
    //     while let Some(x) = cur {
    //         if i == index {
    //             res = Some(&mut x.borrow_mut()._val);
    //             break;
    //         }
    //         cur = x.borrow_mut()._next.as_mut();
    //         i += 1;
    //     }
    //     res
    // }
    // //获取index下标处不可变值
    pub fn get(&self, index: usize) -> Option<std::cell::Ref<DNode<T>>> {
        let mut i = 0;
        let mut cur = self._head.as_ref();
        let mut res = None;
        while let Some(x) = cur {
            if i == index {
                res = Some(x.borrow());
                break;
            }
            // cur = x.borrow().next();
            println!("{:?}", x.borrow().next());
            i += 1;
        }
        res
        // None
    }

    // //头插
    // pub fn add_at_head(&mut self, val: T) {
    //     let new_node = Rc::new(RefCell::new(DNode {
    //         _val: val,
    //         _next: self._head.take(),
    //         _prev: None,
    //     }));
    //     self._head = Some(new_node);
    //     self._len += 1;
    // }

    // //尾插
    // pub fn add_at_tail(&self, val: T) {
    //     let len = self._len;
    //     let mut new_node = Some(Rc::new(RefCell::new(DNode {
    //         _val: val,
    //         _next: None,
    //         _prev: None,
    //     })));
    //     if len == 0 {
    //         self._head = new_node;
    //         self._len += 1;
    //         return;
    //     }
    //     // let head = &mut self._head;
    //     let mut cur = self._head.as_ref();
    //     let mut i = 0;
    //     while let Some(x) = cur {
    //         if i == len - 1 {
    //             x.borrow_mut()._next = new_node;
    //             self._len += 1;
    //             break;
    //         }
    //         cur = x.borrow().next();
    //         i += 1;
    //     }
    // }

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

#[test]
fn test() {}
