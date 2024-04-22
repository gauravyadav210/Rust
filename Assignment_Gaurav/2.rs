// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.


fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut result: Option<usize> = None;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            result = Some(mid);
            right = mid - 1; 
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}

fn main() {
    let arr = [1, 2, 2, 3, 3, 3, 4, 5, 6, 7];
    let target = 3;

    if let Some(index) = find_first_occurrence(&arr, target) {
        println!("The first occurrence of {} is at index {}", target, index);
    } else {
        println!("{} not found in the array", target);
    }
}
