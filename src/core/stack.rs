use crate::core::list::List;

//栈
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<T> {
    _data: List<T>,
}

impl<T> Stack<T> {
    pub fn new(val: T) -> Self {
        Self {
            _data: List::<T>::new(val),
        }
    }
    pub fn push(&mut self, val: T) {
        self._data.add_at_head(val);
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
impl<T: Clone> Stack<T> {
    pub fn pop(&mut self) -> Option<T> {
        self._data.delete_head()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self {
            _data: List::default(),
        }
    }
}

#[test]
fn test() {
    let mut stack = Stack::<i32>::new(789);
    stack.push(123);
    stack.push(456);
    stack.clear();
    println!("stack:{:?}", stack.empty());
}
