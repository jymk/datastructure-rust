use std::cmp::Ordering::{Equal, Greater, Less};
use std::{cell::RefCell, rc::Rc};

use super::dlist::DNode;

type OInnerNode<T> = Option<Rc<RefCell<DNode<T>>>>;

/// 有序链表
#[derive(Debug)]
pub struct OrderedList<T> {
    _head: OInnerNode<T>,
    _len: usize,
}

impl<T> OrderedList<T> {
    pub fn new(t: T) -> Self {
        Self {
            _head: Some(Rc::new(RefCell::new(DNode::new(t)))),
            _len: 1,
        }
    }

    fn _add_at_head(&mut self, new_node: OInnerNode<T>) {
        self._head = new_node;
        self._len += 1;
    }
}

impl<T: Ord> OrderedList<T> {
    pub fn find(&self, t: &T) -> bool {
        let mut cur = self._head.clone();
        while let Some(x) = cur {
            if t.cmp(x.borrow().get_value()) == Equal {
                return true;
            }
            cur = x.borrow().next_cp();
        }
        false
    }
}

impl<T: Ord + Clone> OrderedList<T> {
    pub fn add(&mut self, t: T) {
        let new_node = Rc::new(RefCell::new(DNode::new(t.clone())));
        if self._head.is_none() {
            self._add_at_head(Some(new_node));
            return;
        }

        let mut cur = self._head.clone();
        while let Some(x) = cur {
            let x_borrow = x.borrow().clone();
            let cur_val = x_borrow.get_value();
            match t.cmp(cur_val) {
                Equal => break,
                Less => {
                    // 若prev为none，则插入头；若大于prev，则插入prev后；若小于则继续
                    let prev = x_borrow.prev_cp();
                    if prev == None {
                        *x.borrow_mut().prev_mut() = Some(new_node);
                        self._len += 1;
                        break;
                    }

                    let prev = prev.unwrap();
                    if t.cmp(prev.borrow().get_value()) == Greater {
                        // 设置新节点的前后节点
                        *new_node.borrow_mut().prev_mut() = Some(prev.clone());
                        *new_node.borrow_mut().next_mut() = Some(x.clone());
                        // 设置当前节点的前节点为新节点
                        *x.borrow_mut().prev_mut() = Some(new_node.clone());
                        // 设置前节点的后节点为新节点
                        *prev.borrow_mut().next_mut() = Some(new_node.clone());
                        self._len += 1;
                        break;
                    }
                }
                Greater => {
                    // 若next为none，则插入尾；若小于next，则插入next前；若大于则继续
                    let next = x_borrow.next_cp();
                    if next == None {
                        *x.borrow_mut().next_mut() = Some(new_node);
                        self._len += 1;
                        break;
                    }

                    let next = next.unwrap();
                    if t.cmp(next.borrow().get_value()) == Less {
                        // 设置新节点的前后节点
                        *new_node.borrow_mut().prev_mut() = Some(x.clone());
                        *new_node.borrow_mut().next_mut() = Some(next.clone());
                        // 设置当前节点的后节点为新节点
                        *x.borrow_mut().next_mut() = Some(new_node.clone());
                        // 设置后节点的前节点为新节点
                        *next.borrow_mut().prev_mut() = Some(new_node.clone());
                        self._len += 1;
                        break;
                    }
                }
            }
            cur = x.borrow().next_cp();
        }
    }

    pub fn delete(&mut self, t: &T) {
        let mut cur = self._head.clone();
        if cur.is_none() {
            return;
        }

        while let Some(x) = cur {
            let x_borrow = x.borrow().clone();
            let cur_val = x_borrow.get_value();
            match t.cmp(cur_val) {
                Equal => {
                    let prev = x_borrow.prev_cp();
                    let next = x_borrow.next_cp();

                    if prev.is_none() && next.is_none() {
                        // 说明只有一个节点，置空
                        self._len = 0;
                        self._head = None;
                        break;
                    }

                    self._len -= 1;
                    if prev.is_none() {
                        // 说明x是头结点
                        let next = next.unwrap();
                        *next.borrow_mut().prev_mut() = None;
                        self._head = Some(next.clone());
                        break;
                    }

                    if next.is_none() {
                        // 说明x是尾结点
                        let prev = prev.unwrap();
                        *prev.borrow_mut().next_mut() = None;
                        break;
                    }

                    // 中间节点，将prev的next指为next，将next的prev指为prev
                    let (prev, next) = (prev.unwrap(), next.unwrap());
                    *prev.borrow_mut().next_mut() = Some(next.clone());
                    *next.borrow_mut().prev_mut() = Some(prev.clone());
                    break;
                }
                Less => {
                    // 若小于则继续；若prev为none或者大于prev则break
                    let prev = x_borrow.prev_cp();
                    if prev.is_none() {
                        break;
                    }
                    let prev = prev.unwrap();
                    if t.cmp(prev.borrow().get_value()) == Greater {
                        break;
                    }
                }
                Greater => {
                    // 若大于则继续；若next为none或者小于next则break
                    let next = x_borrow.next_cp();
                    if next.is_none() {
                        break;
                    }
                    let next = next.unwrap();
                    if t.cmp(next.borrow().get_value()) == Less {
                        break;
                    }
                }
            }
            cur = x.borrow().next_cp();
        }
    }
}

impl<T> Default for OrderedList<T> {
    fn default() -> Self {
        Self {
            _head: Option::default(),
            _len: usize::default(),
        }
    }
}

#[test]
fn test() {
    let mut res = OrderedList::default();
    println!("res={:?}", res);
    res.add(1);
    res.add(3);
    res.add(2);
    res.add(4);
    res.add(9);
    res.add(8);
    res.add(8);
    res.delete(&8);
    println!("res={:?}", res);
    println!("has9={:?}", res.find(&9));
    res.delete(&9);
    println!("has9={:?}", res.find(&9));
    println!("res={:?}", res);
    res.delete(&1);
    println!("res={:?}", res);
}
