pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by_key(|word| {
        word.chars()
            .find(|c| c.is_digit(10))
            .unwrap()           
            .to_digit(10) 
            .unwrap() as usize  
    });

    let mut result: Vec<String> = words
        .iter()
        .map(|word| word.chars().filter(|c| !c.is_digit(10)).collect())
        .collect();

    if let Some(first_word) = result.first_mut() {
        if let Some(first_char) = first_word.chars().next() {
            *first_word = first_char.to_ascii_uppercase().to_string() + &first_word[1..];
        }
    }

    result.join(" ")
}


/*
Instructions
Create a function named arrange_phrase, that takes a string literal, sorts the words and returns it. Each word will contain a number that indicates the position of that word.

Expected Functions
pub fn arrange_phrase(phrase: &str) -> String {
}
Your heap allocations will be monitored to ensure that you do not make too many allocations, and that your allocations are reasonably sized.

Usage
Here is a program to test your function

use arrange_it::*;

fn main() {
    println!("{}", arrange_phrase("is2 Thi1s T4est 3a"));
}
And its output

$ cargo run
This is a Test
$
Notions
stack and heap
str
*/