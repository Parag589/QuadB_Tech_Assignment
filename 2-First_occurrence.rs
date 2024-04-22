// 2. Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut result: Option<usize> = None;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target {
            result = Some(mid);
            high = mid - 1; 
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    let sorted_array = vec![1, 2, 3, 4, 4, 4, 5, 6, 7];
    let target = 4;
    
    match first_occurrence(&sorted_array, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
