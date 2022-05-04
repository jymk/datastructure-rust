pub mod bubble;
pub mod merge;
pub mod quick;

#[test]
fn test() {
    let mut datas = vec![49, 38, 65, 97, 76, 13, 27, 49];
    let mut datas = vec![26, 5, 37, 1, 61, 11, 59, 15, 48, 19];
    // bubble::bubble_sort(&mut datas);
    // quick::quick_sort(&mut datas);
    merge::merge_sort(&mut datas);
    println!("datas:{:?}", datas);
}
