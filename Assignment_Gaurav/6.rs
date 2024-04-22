//Implement a function that finds the longest common prefix of a given set of strings.



fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = strings[0].clone();
    
    for string in &strings[1..] {
        while !string.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    
    prefix
}

fn main() {
    let strings1 = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let strings2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

    println!("Longest common prefix of strings1: {}", longest_common_prefix(&strings1));
    println!("Longest common prefix of strings2: {}", longest_common_prefix(&strings2));
}
