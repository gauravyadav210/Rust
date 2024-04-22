//Reverse a string in Rust

fn reverse_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.reverse();
    chars.into_iter().collect()
}

fn main() {
    let s = "Hello, world!";
    let reversed = reverse_string(s);
    println!("Original string: {}", s);
    println!("Reversed string: {}", reversed);
}
