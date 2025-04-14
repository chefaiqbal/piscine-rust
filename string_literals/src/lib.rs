pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}


pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap()
}

/*
Instructions
Create the following functions:

is_empty: that returns true if the string is empty.
is_ascii: that returns true if all characters are within the ASCII range.
contains: that returns true if the string contains the given pattern.
split_at: that divides a string in two returning a tuple.
find: that returns the index of the first character of a given string that matches the pattern.
Expected functions
pub fn is_empty(v: &str) -> bool {
}

pub fn is_ascii(v: &str) -> bool {
}

pub fn contains(v: &str, pat: &str) -> bool {
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
}

pub fn find(v: &str, pat: char) -> usize {
}
You mustn't allocate to the heap in any of these functions.

Usage
Here is a program to test your function

use string_literals::*;

fn main() {
    println!("{}", is_empty(""));
    println!("{}", is_ascii("rust"));
    println!("{}", contains("rust", "ru"));
    println!("{:?}", split_at("rust", 2));
    println!("{}", find("rust", 'u'));
}
And its output

$ cargo run
true
true
true
("ru", "st")
1
$
Notions
stack and heap
Literals
*/