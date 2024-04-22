// 12. Find the maximum subarray sum in Rust

use std::cmp::max;

struct MaxSubarray;

impl MaxSubarray {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        nums.iter().fold((i32::MIN, 0), |(max_sum, sum), &num| {
            let new_sum = max(sum + num, num);
            (max(max_sum, new_sum), new_sum)
        }).0
    }
}

fn main() {
    let nums = vec![-2, 1, -1, 2, 1, -5, 4];
    let result = MaxSubarray::max_sum(nums);
    println!("The maximum sum of a contiguous subarray is {}", result);
}
