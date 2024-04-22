//Given a sorted array of integers, implement a function that returns the median of the array.


fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (arr[mid_left] + arr[mid_right]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5, 6];
    
    println!("Median of arr1: {}", find_median(&arr1));
    println!("Median of arr2: {}", find_median(&arr2));
}
