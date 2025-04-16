pub fn capitalize_first(input: &str) -> String {
    if let Some(first_char) = input.chars().next() {
        first_char.to_ascii_uppercase().to_string() + &input[first_char.len_utf8()..]
    } else {
        String::new()
    }
}

pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            if let Some(first_char) = word.chars().next() {
                first_char.to_ascii_uppercase().to_string() + &word[first_char.len_utf8()..]
            } else {
                String::new()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            }
        })
        .collect()
}

/*
Instructions
Complete the capitalize_first function which converts the first letter of the string to uppercase.

Complete the title_case function which converts the first letter of each word in a string to uppercase.

Complete the change_case function which converts the uppercase letters of a string into lowercase, and the lowercase to uppercase.

Expected Functions
pub fn capitalize_first(input: &str) -> String {
}

pub fn title_case(input: &str) -> String {
}

pub fn change_case(input: &str) -> String {
}
Usage
Here is a program to test your functions.

use capitalizing::*;

fn main() {
    println!("{}", capitalize_first("joe is missing"));
    println!("{}", title_case("jill is leaving A"));
    println!("{}", change_case("heLLo THere"));
}
And its output

$ cargo run
Joe is missing
Jill Is Leaving A
HEllO thERE
$
*/