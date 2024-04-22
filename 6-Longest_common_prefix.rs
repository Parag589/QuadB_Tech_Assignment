//6. Implement a function that finds the longest common prefix of a given set of strings.

fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();
    let first_str = strings[0];

    for (i, c) in first_str.chars().enumerate() {
        if strings.iter().all(|s| s.chars().nth(i) == Some(c)) {
            prefix.push(c);
        } else {
            break; 
        }
    }

    prefix
}

fn main() {
    let strings = ["flower", "flow", "flight"];

    println!("Longest common prefix of {:?}: {}", strings, longest_common_prefix(&strings));
}
