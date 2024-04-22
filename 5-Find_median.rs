// 5. Given a sorted array of integers, implement a function that returns the median of the array.

fn find_median(arr: &[i32]) -> Option<f64> {
    let len = arr.len();
    if len == 0 {
        return None; 
    }

    if len % 2 != 0 {
        return Some(arr[len / 2] as f64);
    }

    let mid_index1 = len / 2 - 1;
    let mid_index2 = len / 2;
    let median = (arr[mid_index1] + arr[mid_index2]) as f64 / 2.0;
    Some(median)
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5]; // Odd length array
    let arr2 = [1, 2, 3, 4];     // Even length array
    let arr3: [i32; 0] = [];     // Empty array

    if let Some(median) = find_median(&arr1) {
        println!("Median of {:?}: {}", arr1, median);
    } else {
        println!("Array is empty");
    }

    if let Some(median) = find_median(&arr2) {
        println!("Median of {:?}: {}", arr2, median);
    } else {
        println!("Array is empty");
    }

    if let Some(median) = find_median(&arr3) {
        println!("Median of {:?}: {}", arr3, median);
    } else {
        println!("Array is empty");
    }
}
