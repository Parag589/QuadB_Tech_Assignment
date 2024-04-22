// 1. Implement a function that checks whether a given string is a palindrome or not.

fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase();
    let input_chars: Vec<char> = input.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed_chars: Vec<char> = input_chars.iter().rev().cloned().collect();
    input_chars == reversed_chars
}

fn main() {
    let test_string1 = "A man, a plan, a canal, Panama";
    let test_string2 = "hello";
    
    println!("Is '{}' a palindrome? {}", test_string1, is_palindrome(test_string1));
    println!("Is '{}' a palindrome? {}", test_string2, is_palindrome(test_string2));
}
