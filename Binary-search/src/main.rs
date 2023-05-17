use std::time::Instant;

fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    None
}

fn main() {
    let arr = [2, 4, 6, 8, 10, 12, 14];
    let target = 8;

    let start = Instant::now();

    match binary_search(&arr, target) {
        Some(index) => println!("Element {} found at index {}", target, index),
        None => println!("Element {} not found", target),
    }

    let end = Instant::now();
    let duration = end.duration_since(start);

    println!("Compile time: {:?}", duration);
}
