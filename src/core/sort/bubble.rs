//冒泡排序
pub fn bubble_sort(datas: &mut Vec<i32>) {
    let len = datas.len();
    for i in 0..len - 1 {
        for j in 0..len - i - 1 {
            if datas[j] > datas[j + 1] {
                let tmp = datas[j];
                datas[j] = datas[j + 1];
                datas[j + 1] = tmp;
            }
        }
    }
}
