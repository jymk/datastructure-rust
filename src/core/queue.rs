use crate::core::dlist::DList;

//队列
struct Queue<T> {
    _data: DList<T>,
}

impl<T> Queue<T> {
    pub fn new(val: T) -> Self {
        Self {
            _data: DList::new(val),
        }
    }
    //计数
    pub fn count(&self) -> usize {
        self._data.len()
    }
    //是否为空
    pub fn empty(&self) -> bool {
        self._data.len() == 0
    }
}
impl<T: Clone> Queue<T> {
    //入队
    pub fn enqueue(&mut self, val: T) {
        self._data.add_at_tail(val);
    }
    //出队
    pub fn dequeue(&mut self) -> Option<T> {
        self._data.delete_head()
    }
    //查看头
    pub fn peek(&self) -> Option<T> {
        self._data.get(0)
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self {
            _data: DList::default(),
        }
    }
}
