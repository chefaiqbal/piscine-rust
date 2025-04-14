pub fn first_subword(mut s: String) -> String {
    let mut result = String::new();
    let mut chars = s.chars();

    if let Some(first_char) = chars.next() {
        result.push(first_char);
    }

   
    while let Some(c) = chars.next() {
        if c.is_uppercase() || c == '_' {
            break;
        }
        result.push(c);
    }

    result
}




/*
Instruction

Create a function named first_subword, that moves a string (takes ownership of it) and returns the first sub-word in it. You can mutate the original string. It should work for camelCase, PascalCase, and snake_case.
Expected functions

pub fn first_subword(mut s: String) -> String {
}

Usage

Here is a program to test your function:

use ownership::first_subword;

fn main() {
    let s1 = "helloWorld";
    let s2 = "snake_case";
    let s3 = "CamelCase";
    let s4 = "just";

    println!("first_subword({}) = {}", s1, first_subword(s1.to_owned()));
    println!("first_subword({}) = {}", s2, first_subword(s2.to_owned()));
    println!("first_subword({}) = {}", s3, first_subword(s3.to_owned()));
    println!("first_subword({}) = {}", s4, first_subword(s4.to_owned()));
}

And its output:

$ cargo run
first_subword(helloWorld) = hello
first_subword(snake_case) = snake
first_subword(CamelCase) = Camel
first_subword(just) = just
$

Notions

    ownership

*/