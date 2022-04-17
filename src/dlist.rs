use std::{
    cell::RefCell,
    fmt::{Debug, Display},
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

impl<T: Clone> DList<T> {
    pub fn new() -> Self {
        DList {
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

    //这里不好改成循环，参考https://blog.csdn.net/weixin_51560951/article/details/122310256
    fn _get_index_node(
        &self,
        node: Rc<RefCell<DNode<T>>>,
        cur: usize,
        index: usize,
    ) -> Option<Rc<RefCell<DNode<T>>>> {
        // println!("node:{:?}, cur:{}, index:{}", node.clone(), cur, index);
        if cur >= index {
            return Some(node);
        }
        if let Some(n) = &node.borrow_mut()._next {
            let a = self._get_index_node(n.clone(), cur + 1, index);
            return a.clone();
        }
        return None;
    }

    //获取index下标处节点
    pub fn get_node(&self, index: usize) -> Option<Rc<RefCell<DNode<T>>>> {
        if let Some(h) = self._head.as_ref() {
            // println!("h:{:?}, index:{}", h.clone(), index);
            return self._get_index_node(h.clone(), 0, index);
        }
        None
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
        self.add_at_index(self._len, val);
    }

    //下标插
    pub fn add_at_index(&mut self, index: usize, val: T) {
        let len = self._len;
        if index > len {
            return;
        }
        if index == 0 || len == 0 {
            self.add_at_head(val.clone());
            return;
        }
        let mut new_node = DNode::new(val);
        let mod_prev_node = self.get_node(index - 1);
        // println!("mod_node:{:?}", mod_prev_node);
        if let Some(x) = mod_prev_node {
            //新节点赋值
            new_node._next = x.borrow()._next.clone();
            new_node._prev = Some(x.clone());
            let rc = Rc::new(RefCell::new(new_node));
            //右节点的prev赋值为新节点
            if let Some(y) = &x.borrow_mut()._next {
                y.borrow_mut()._prev = Some(rc.clone());
            }
            //左节点的next赋值为新节点
            x.borrow_mut()._next = Some(rc.clone());
            self._len += 1;
        }
    }

    //删头
    pub fn delete_head(&mut self) -> Option<T> {
        if let Some(ref mut h) = self._head {
            let h_val = h.borrow()._val.clone();
            let head = h.borrow_mut()._next.take();
            if let Some(x) = &head {
                x.borrow_mut()._prev = None;
            }
            self._head = head;
            self._len -= 1;
            return Some(h_val);
        } else {
            None
        }
    }
    // //删下标
    pub fn delete_at_index(&mut self, index: usize) -> Option<T> {
        //下标为0
        if index == 0 {
            return self.delete_head();
        }
        if let Some(l) = self.get_node(index - 1) {
            let mid_node = l.borrow_mut()._next.take();
            if let Some(m) = &mid_node {
                let mid_val = m.borrow()._val.clone();
                let right_node = m.borrow_mut()._next.take();
                if let Some(r) = &right_node {
                    //设置右节点的prev为左节点
                    r.borrow_mut()._prev = Some(l.clone());
                    //设置左节点的next为右节点
                    l.borrow_mut()._next = Some(r.clone());
                } else {
                    //设置左节点的next为None
                    l.borrow_mut()._next = None;
                }
                self._len -= 1;
                return Some(mid_val);
            }
        }
        None
    }

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

impl<T: Clone> Default for DList<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 不实现DNode的打印方法，会使得打印循环引用而爆栈
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
        write!(f, "None")
    }
}
impl<T: Debug> Debug for DNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} => ", self._val)?;
        if let Some(node) = &self._next {
            return Debug::fmt(&node.borrow(), f);
        }
        write!(f, "None")
    }
}

#[test]
fn test() {
    let mut dl = DList::<i32>::new();
    //11 -> 10 -> 9 -> 4 -> 3
    dl.add_at_head(3);
    dl.add_at_head(4);
    dl.add_at_head(9);
    dl.add_at_head(10);
    dl.add_at_head(11);
    dl.add_at_tail(5);
    dl.add_at_tail(7);
    dl.add_at_tail(6);
    dl.add_at_index(0, 1);
    dl.add_at_index(2, 2);
    dl.add_at_index(dl._len, 8);
    println!("dl:{}, len:{}", dl, dl.len());
    dl.delete_at_index(3);
    println!("dl:{}, len:{}", dl, dl.len());
    dl.delete_at_index(0);
    println!("dl:{}, len:{}", dl, dl.len());
    dl.delete_at_index(dl.len() - 3);
    println!("dl:{}, len:{}", dl, dl.len());
    // println!("dl:{}", dl);
}
