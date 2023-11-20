/*fn bubble_sort(arr: &mut [i32; 6]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}*/

/*fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);

    let mut sorted = vec![0; arr.len()];
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            sorted[k] = left[i];
            i += 1;
        } else {
            sorted[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        sorted[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        sorted[k] = right[j];
        j += 1;
        k += 1;
    }

    arr.copy_from_slice(&sorted);
}

fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}


fn main() {
    let mut array = [64, 64, 25, 12, 22, 11];
    
    bubble_sort(&mut array);
    
    println!("Datos ordenados: {:?}", array);
}*/

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
    i
}

fn quick_sort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot = partition(arr, low, high);

        if pivot > 0 {
            quick_sort(arr, low, pivot - 1);
        }

        quick_sort(arr, pivot + 1, high);
    }
}

fn main() {
    let mut data = vec![64, 64, 25, 12, 22, 11];
    let len = data.len();
    
    println!("Datos sin ordenar: {:?}", data);
    
    quick_sort(&mut data, 0, len - 1);
    println!("Datos ordenados con Quick Sort: {:?}", data);
}


