// 9. Reverse a string in Rust

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let input_string = "hello world";
    let reversed_string = reverse_string(input_string);
    println!("Original string: {}", input_string);
    println!("Reversed string: {}", reversed_string);
}
