use crate::errs::OUT_OF_RANGE;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// 链表
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<T> {
    _head: InnerNode<T>,
    _len: usize,
}

type InnerNode<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T> {
    _val: T,
    _next: InnerNode<T>,
}
impl<T: Clone> Node<T> {
    pub fn get_value(&self) -> &T {
        &self._val
    }

    //下一个节点
    pub fn next(&self) -> Option<&Self> {
        match &self._next {
            Some(v) => Some(&*v),
            None => None,
        }
    }
    //下一个可变节点
    pub fn next_mut(&mut self) -> Option<&mut Self> {
        match self._next.as_mut() {
            Some(v) => Some(v.as_mut()),
            None => None,
        }
    }
    //当前可变值
    pub fn get_mut(&mut self) -> &mut T {
        &mut self._val
    }
}

impl<T: Clone> List<T> {
    pub fn new() -> Self {
        List {
            _head: None,
            _len: usize::default(),
        }
    }
    //new并在尾部增加一个值
    pub fn new_with_val(val: T) -> Self {
        let mut tmp = Self::new();
        tmp.add(val);
        tmp
    }
    //在尾部增加一个值
    pub fn add(&mut self, val: T) {
        self.add_at_tail(val);
    }
    //获取index下标处可变节点
    pub fn get_node_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        let mut i = 0;
        let mut cur = self._head.as_mut();
        let mut res = None;
        while let Some(x) = cur {
            if i == index {
                res = Some(x.as_mut());
                break;
            }
            cur = x._next.as_mut();
            i += 1;
        }
        res
    }
    //获取index下标处可变值
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if let Some(x) = self.get_node_mut(index) {
            Some(&mut x._val)
        } else {
            None
        }
    }
    //获取index下标处不可变值
    pub fn get(&self, index: usize) -> Option<&T> {
        let mut i = 0;
        let mut cur = self._head.as_ref();
        let mut res = None;
        while let Some(x) = cur {
            if i == index {
                res = Some(&x._val);
                break;
            }
            cur = x._next.as_ref();
            i += 1;
        }
        res
    }

    //头插
    pub fn add_at_head(&mut self, val: T) {
        let new_node = Box::new(Node {
            _val: val,
            _next: self._head.take(),
        });
        self._head = Some(new_node);
        self._len += 1;
    }

    //尾插
    pub fn add_at_tail(&mut self, val: T) {
        let len = self._len;
        let mut new_node = Some(Box::new(Node {
            _val: val,
            _next: None,
        }));
        if len == 0 {
            self._head = new_node;
            self._len += 1;
            return;
        }
        if let Some(x) = self.get_node_mut(len - 1) {
            x._next = new_node;
            self._len += 1;
        }
    }

    //下标插
    pub fn add_at_index(&mut self, index: usize, val: T) {
        let len = self._len;
        if index > len {
            return;
        }
        //下标为0 或 长度为0
        if len == 0 || index == 0 {
            self.add_at_head(val.clone());
            return;
        }
        let mut new_node = Some(Box::new(Node {
            _val: val,
            _next: None,
        }));
        if let Some(x) = self.get_node_mut(index - 1) {
            let tmp = new_node.as_mut();
            if let Some(n) = tmp {
                n.as_mut()._next = x._next.clone();
            }
            x._next = new_node;
            self._len += 1;
        }
    }

    //删头
    pub fn delete_head(&mut self) {
        if let Some(x) = self._head.as_mut() {
            self._head = x._next.clone();
            self._len -= 1;
        }
    }
    //删下标
    pub fn delete_at_index(&mut self, index: usize) {
        let len = self._len;
        let mut cur = self._head.as_mut();
        //下标为0
        if len > 0 && index == 0 {
            self.delete_head();
            return;
        }
        let mut i = 0;
        while let Some(x) = cur {
            if i == index - 1 {
                let mid = x._next.as_mut();
                if let Some(m) = mid {
                    let mut right = m._next.as_mut();
                    if let Some(r) = right {
                        x.as_mut()._next = Some(r.clone());
                        self._len -= 1;
                    }
                }
                break;
            }
            cur = x._next.as_mut();
            i += 1;
        }
    }

    //长度
    pub fn len(&self) -> usize {
        self._len
    }

    //头节点
    pub fn next(&self) -> Option<&Node<T>> {
        match &self._head {
            Some(v) => Some(&v.deref()),
            None => None,
        }
    }
    //可变头节点
    pub fn next_mut(&mut self) -> Option<&mut Node<T>> {
        match self._head.as_mut() {
            Some(v) => Some(v.as_mut()),
            None => None,
        }
    }
    //清空
    pub fn clear(&mut self) {
        self._head = None;
        self._len = 0;
    }
    //反转
    pub fn reverse(&mut self) {
        let mut node = &self._head;
        let mut cur = None;
        while let Some(x) = node.as_ref() {
            cur = Some(Box::new(Node {
                _val: x._val.clone(),
                _next: cur,
            }));
            node = &x._next;
        }
        self._head = cur;
    }
}
impl<T: Clone> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect(OUT_OF_RANGE)
    }
}

impl<T: Clone> IndexMut<usize> for List<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect(OUT_OF_RANGE)
    }
}

//只能从下标1开始
impl<'a, T> Iterator for &'a Node<T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self._next.as_ref() {
            Some(v) => {
                *self = v.deref();
                Some(v.deref())
            }
            None => None,
        }
    }
}
// impl<'a, T: Clone> Iterator for &'a mut List<T> {
//     type Item = &'a mut Node<T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self._head.as_ref() {
//             Some(v) => {
//                 **self = List::new_with_head(&v._next);
//                 Some(v.deref_mut())
//             }
//             None => None,
//         }
//     }
// }

// impl<'a, T> IntoIterator for &'a Node<T> {
//     type Item = &'a T;
//     type IntoIter = &'a Node<T>;
//     fn into_iter(self) -> Self::IntoIter {
//         self._next.unwrap().deref()
//     }
// }

impl<T: Clone> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test() {
    let mut list = List::<i32>::new();
    // println!("len:{}, list:{:?}", list._len, list);
    list.add_at_index(0, 9);
    list.add_at_index(0, 7);
    list.add_at_index(2, 8);
    // list.add(0);
    // list.add(6);
    // list.add_at_tail(7);
    // list.add(2);
    // list.add(3);
    // list.add(4);
    // list.add(5);
    // let len = list._len;
    println!("len:{}, list:{:?}", list.len(), list);
    // let cur = list._head.as_ref();
    // if let Some(x) = cur {
    //     for l in x.deref() {
    //         println!("val:{}", l.get_value());
    //     }
    // }
    // println!("len:{}, list:{:?}", list._len, list.get_mut(2));
    // list.reverse();
    // println!("len:{}, list:{:?}", list._len, list);

    // let a = Rc::new(5);
    // let b = a.clone();
    // println!("a:{:p}, b:{:p}", a, b);
}
