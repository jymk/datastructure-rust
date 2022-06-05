use std::{cell::RefCell, rc::Rc};

//Option<Box<Node>>结构的节点
pub trait BoxNode<T> {
    type U: BoxNode<T>;

    fn get_node(&self, t: &Option<&T>) -> Option<&Self::U>;
}

//Option<Box<Node>>结构本体
pub trait BoxEntity<T> {
    type U: BoxNode<T>;

    fn get_node(&self, t: &T) -> Option<&Self::U>;
}
