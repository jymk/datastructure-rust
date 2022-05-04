//快排
pub fn quick_sort(datas: &mut Vec<i32>) {
    let len = datas.len();
    if len <= 1 {
        return;
    }
    _qsort(datas, 0, len - 1);
}

fn _qsort(datas: &mut Vec<i32>, left: usize, right: usize) {
    let tmp = datas[left];
    let mut p = left;
    let (mut i, mut j) = (left, right);
    while i <= j {
        while j > 0 && j >= p && datas[j] >= tmp {
            j -= 1;
        }
        if j >= p {
            datas[p] = datas[j];
            p = j;
        }
        while i <= p && datas[i] <= tmp {
            i += 1;
        }
        if i <= p {
            datas[p] = datas[i];
            p = i;
        }
    }
    datas[p] = tmp;
    // println!(
    //     "p:{}, left:{}, right: {}, datas:{:?}",
    //     p, left, right, datas
    // );
    println!("datas:{:?}", datas);
    if p - left > 1 {
        _qsort(datas, left, p - 1);
    }
    if right - p > 1 {
        _qsort(datas, p + 1, right);
    }
}
