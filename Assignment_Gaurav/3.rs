//Given a string of words, implement a function that returns the shortest word in the string.




fn find_shortest_word(sentence: &str) -> Option<&str> {
    sentence.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    
    if let Some(shortest_word) = find_shortest_word(sentence) {
        println!("The shortest word in the sentence is: {}", shortest_word);
    } else {
        println!("No words found in the sentence");
    }
}
