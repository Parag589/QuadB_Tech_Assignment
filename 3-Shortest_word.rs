// 3. Given a string of words, implement a function that returns the shortest word in the string.

fn shortest_word(input: &str) -> Option<&str> {
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut shortest_word: Option<&str> = None;
    let mut shortest_length = usize::MAX;
    
    for word in words {
        let word_length = word.len();
        if word_length < shortest_length {
            shortest_length = word_length;
            shortest_word = Some(word);
        }
    }
    
    shortest_word
}

fn main() {
    let input_string = "This is a test string with some short words";
    if let Some(shortest) = shortest_word(input_string) {
        println!("Shortest word: {}", shortest);
    } else {
        println!("No words found in the input string");
    }
}
