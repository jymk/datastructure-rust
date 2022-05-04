// use std::ops::Index;
use crate::{common::errs::OUT_OF_RANGE, core::list::List};
use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
};

//扩容因子
const _FACTOR: f32 = 0.75;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MyHashMap<K, V> {
    _data: Vec<Option<List<(K, V)>>>,
    _len: usize,
    _cap: usize,
}

impl<K: Hash + Eq + Clone, V: Clone> MyHashMap<K, V> {
    //根据k获取下标
    fn _get_index(&self, k: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        k.hash(&mut hasher);
        hasher.finish() as usize % self._cap
    }
    //默认设置容量为4
    pub fn new() -> Self {
        Self::with_cap(4)
    }
    //new并设置容量
    pub fn with_cap(cap: usize) -> Self {
        let mut v = Vec::<Option<List<(K, V)>>>::default();
        for _ in 0..cap {
            v.push(None);
        }
        MyHashMap {
            _data: v,
            _len: 0,
            _cap: cap,
        }
    }
    pub fn put(&mut self, k: &K, v: V) {
        let index = self._get_index(k);
        // println!("index:{}, k:{:?}, v:{:?}", index, k, v);
        //若k对应的v存在
        let data = self._data[index].as_mut();
        match data {
            Some(d) => {
                //若对应位置存在数据，加入其中的链表
                //若有此k
                let mut cur = d.next_mut();
                while let Some(c) = cur {
                    if c.get_value().0 == *k {
                        c.get_mut().1 = v;
                        return;
                    }
                    cur = c.next_mut();
                }
                //若无此k
                d.add((k.clone(), v));
            }
            None => {
                //若不存在, 插入
                //若插入超过因子，扩容
                if self._len as f32 / self._cap as f32 >= _FACTOR {
                    let mut new_data = Self::with_cap(self._cap * 2);
                    new_data._len = self._len;
                    for d in &self._data {
                        //扩容时下标要重新生成
                        if let Some(dd) = d {
                            let mut cur = dd.next();
                            while let Some(c) = &cur {
                                let val = c.get_value().clone();
                                //此处put递归调用理论上来说不会经过判断是否达到扩容因子
                                new_data.put(&val.0, val.1);
                                cur = c.next();
                            }
                        }
                    }
                    self._data.clear();
                    *self = new_data;
                }
                //插入
                let mut node = List::<(K, V)>::new((k.clone(), v));
                self._data[index] = Some(node);
                self._len += 1;
            }
        }
    }
    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        let index = self._get_index(k);
        let data = &mut self._data[index];
        // println!("index:{}, k:{:?}", index, k);
        // println!("index:{}, k:{:?}, data:{:?}", index, k, data);
        match data {
            Some(d) => {
                let head = d.next_mut();
                //若head不存在设为None
                let mut res = None;
                //若head存在且head的key与之相等
                if let Some(h) = head {
                    if h.get_mut().0 == *k {
                        return Some(&mut h.get_mut().1);
                    }
                    //否则进行head持续next直到为none
                    let mut cur = h.next_mut();
                    while let Some(v) = cur {
                        if v.get_value().0 == *k {
                            res = Some(&mut v.get_mut().1);
                            break;
                        }
                        cur = v.next_mut();
                    }
                }
                res
            }
            None => None,
        }
    }
    pub fn get(&self, k: &K) -> Option<&V> {
        let index = self._get_index(k);
        let data = &self._data[index];
        // println!("index:{}, k:{:?}", index, k);
        // println!("index:{}, k:{:?}, data:{:?}", index, k, data);
        match data {
            Some(d) => {
                let head = d.next();
                //若head不存在设为None
                let mut res = None;
                //若head存在且head的key与之相等
                if let Some(h) = head {
                    let tmp = h.get_value();
                    if tmp.0 == *k {
                        return Some(&tmp.1);
                    }
                    //否则进行head持续next直到为none
                    let mut cur = h.next();
                    while let Some(v) = cur {
                        if v.get_value().0 == *k {
                            res = Some(&v.get_value().1);
                            break;
                        }
                        cur = v.next();
                    }
                }
                res
            }
            None => None,
        }
    }

    pub fn remove(&mut self, k: &K) {
        let index = self._get_index(k);
        let data = self._data[index].as_mut();
        match data {
            Some(v) => {
                let mut i = 0;
                let mut head = v.next_mut();
                while let Some(h) = head {
                    if h.get_value().0 == *k {
                        let mut left = v.get_node_mut(i - 1);
                        if let Some(l) = left {
                            if let Some(m) = l.next_mut() {
                                if let Some(r) = m.next_mut() {
                                    *m = r.clone();
                                }
                            }
                        }
                        break;
                    }
                    head = h.next_mut();
                    i += 1;
                }
            }
            None => return,
        }
    }
    pub fn remove_all(&mut self) {
        self._len = 0;
        self._cap = 0;
        self._data.clear();
    }
}

impl<K, V> Default for MyHashMap<K, V> {
    fn default() -> Self {
        Self {
            _data: Vec::default(),
            _len: usize::default(),
            _cap: usize::default(),
        }
    }
}

impl<K: Hash + Eq + Clone, V: Clone> Index<&K> for MyHashMap<K, V> {
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.get(index).expect(OUT_OF_RANGE)
    }
}
impl<K: Hash + Eq + Clone, V: Clone> IndexMut<&K> for MyHashMap<K, V> {
    fn index_mut(&mut self, index: &K) -> &mut Self::Output {
        self.get_mut(index).expect(OUT_OF_RANGE)
    }
}
#[test]
fn test() {
    let mut hm = MyHashMap::<i32, i32>::new();
    hm.put(&7, 9);
    hm.put(&8, 19);
    hm.put(&9, 29);
    hm.put(&5, 39);
    hm.put(&56, 39);
    hm.put(&78, 39);
    hm.put(&79, 39);
    hm.put(&80, 39);
    hm.put(&128, 39);
    hm.put(&1288, 39);
    hm.put(&12889, 39);
    // let idx = hm.get(&7);
    println!("hm: {:?}", hm);
    println!(
        "1: {:?},2: {:?},3: {:?},4: {:?},",
        hm.get(&7),
        hm.get(&8),
        hm.get(&9),
        hm.get(&5)
    );
}
