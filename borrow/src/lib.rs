pub fn str_len(s: &str ) -> usize {
    s.chars().count()
}


/*
Instructions
Create a function named str_len, you'll need to complete the function signature. Your function should accept a borrowed string, and return its length (in characters).

Expected functions
pub fn str_len(s: ) -> usize {
}
Usage
Here is a possible program to test your function:

use borrow::*;

fn main() {
	let s = "hello";
	let s1 = "camelCase".to_string();
	let s2 = "olá!";

	println!("\tstr_len(\"{}\") = {}", s, str_len(s));
	println!("\tstr_len(\"{}\") = {}", s1, str_len(&s1));
	println!("\tstr_len(\"{}\") = {}", s2, str_len(s2));
}
And its output:

$ cargo run
str_len("hello") = 5
str_len("camelCase") = 9
str_len("olá!") = 4
$
Notions
Primitive Type str
*/