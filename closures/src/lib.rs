
use std::iter::Iterator;

pub fn first_fifty_even_square() -> Vec<i32> {
    (1..101).filter(|x| x % 2 == 0).map(|x| x * x).collect()
}

/*
Instructions
Using closures and iterators create a function, that returns the first 50 even numbers squared in a Vec<i32>.

Expected Functions
fn first_fifty_even_square() -> Vec<i32> {

}
Usage
Here is a program to test your function.

use closures::*;

fn main() {
	println!("Hello, world!");
	let v1 = first_fifty_even_square();

	println!("All elements in {:?}, len = {}", v1, v1.len());
}
And its output:

$ cargo run
All elements in [4, 16, 36, ..., 10000], len = 50
$
Notions
Iterators and Closures
*/