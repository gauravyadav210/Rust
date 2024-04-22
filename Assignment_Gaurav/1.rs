//Implement a function that checks whether a given string is a palindrome or not.


fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase(); 
    let reversed_input: String = input.chars().rev().collect(); 
    input == reversed_input 
}

fn main() {
    let test_string1 = "racecar";
    let test_string2 = "hello";
    
    println!("Is '{}' a palindrome? {}", test_string1, is_palindrome(test_string1));
    println!("Is '{}' a palindrome? {}", test_string2, is_palindrome(test_string2));
}
