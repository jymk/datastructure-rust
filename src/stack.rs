use crate::list::List;

//栈
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<T> {
    _data: List<T>,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack {
            _data: List::<T>::default(),
        }
    }
    pub fn push(&mut self, val: T) {
        self._data.add_at_head(val);
    }
    pub fn pop(&mut self) {
        self._data.delete_head();
    }
    //查看栈顶
    pub fn peek(&self) -> Option<&T> {
        self._data.get(0)
    }
    //清空栈
    pub fn clear(&mut self) {
        self._data.clear();
    }
    //数量
    pub fn count(&self) -> usize {
        self._data.len()
    }
    //为空返回true
    pub fn empty(&self) -> bool {
        self._data.len() == 0
    }
}

impl<T: Clone> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test() {
    let mut stack = Stack::<i32>::new();
    stack.push(123);
    stack.push(456);
    stack.clear();
    println!("stack:{:?}", stack.empty());
}
