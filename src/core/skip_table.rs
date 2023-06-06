use std::{cell::RefCell, rc::Rc};

use super::ordered_list::OrderedList;

pub struct SkipTable<T> {
    _head: SkipInnerNode<T>,
    _len: usize,
}

type SkipInnerNode<T> = Option<Rc<RefCell<SkipNode<T>>>>;

struct SkipNode<T> {
    _val: T,
    _next: SkipInnerNode<T>,
    _down: SkipInnerNode<T>,
}

impl<T> SkipNode<T> {}

impl<T> SkipTable<T> {}
