//Implement a function that returns the kth smallest element in a given array.



fn partition(arr: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = arr[right];
    let mut i = left;

    for j in left..right {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, right);
    i
}

fn quickselect(arr: &mut [i32], left: usize, right: usize, k: usize) -> i32 {
    if left == right {
        return arr[left];
    }

    let pivot_index = partition(arr, left, right);

    if k == pivot_index {
        arr[k]
    } else if k < pivot_index {
        quickselect(arr, left, pivot_index - 1, k)
    } else {
        quickselect(arr, pivot_index + 1, right, k)
    }
}

fn kth_smallest(arr: &mut [i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }

    Some(quickselect(arr, 0, arr.len() - 1, k - 1))
}

fn main() {
    let mut arr = [7, 10, 4, 3, 20, 15];
    let k = 3;

    if let Some(kth_smallest_element) = kth_smallest(&mut arr, k) {
        println!("The {}th smallest element is: {}", k, kth_smallest_element);
    } else {
        println!("Invalid input: k is larger than the array size.");
    }
}
