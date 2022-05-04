use std::cmp::Ordering;

//归并排序
pub fn merge_sort(array: &mut [i32]) {
    let mid = array.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut array[..mid]);
    merge_sort(&mut array[mid..]);
    println!("datas:{:?}", array);
    merge(array, mid);
}

fn merge(array: &mut [i32], mid: usize) {
    let mut new_array = vec![]; //use vec to keep sorted value
    let mut j = 0;
    let mut k = mid;
    for i in 0..array.len() {
        if k == array.len() || j == mid {
            //if left or right are all selected
            break;
        }
        if array[j] < array[k] {
            new_array.push(array[j]);
            j += 1;
        } else {
            new_array.push(array[k]);
            k += 1;
        }
    }
    match (j.cmp(&mid), k.cmp(&array.len())) {
        (Ordering::Equal, Ordering::Equal) => (),
        (Ordering::Less, Ordering::Equal) => {
            for i in j..mid {
                new_array.push(array[i]);
            }
        }
        (Ordering::Equal, Ordering::Less) => {
            for i in k..array.len() {
                new_array.push(array[i]);
            }
        }
        _ => (),
    }

    for i in 0..array.len() {
        array[i] = new_array[i];
    }
}
