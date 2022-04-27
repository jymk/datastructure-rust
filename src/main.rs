#![allow(unused)]

mod binary_tree;
mod bst;
mod common;
mod dlist;
mod errs;
mod hm;
mod list;
mod queue;
mod stack;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    let mut ts = TestStruct::<i32>::new();
    ts.field1 = 5;
    ts.field2 = 6;
    for t in ts {
        println!("t:{}", t);
    }
}

#[derive(Debug)]
struct TestStruct<T> {
    field1: i32,
    field2: T,
}
impl<T: Default> TestStruct<T> {
    fn new() -> Self {
        TestStruct {
            field1: 0,
            field2: T::default(),
        }
    }
}
impl<'a, T> Iterator for &'a TestStruct<T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        Some(&self.field2)
    }
}
impl<T: Clone> Iterator for TestStruct<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.field2.clone())
    }
}
// impl<'a, T> Iterator for &'a mut TestStruct<T> {
//     type Item = &'a mut T;
//     fn next(&'a mut self) -> Option<Self::Item> {
//         Some(&mut self.field2)
//     }
// }
