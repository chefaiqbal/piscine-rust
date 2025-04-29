pub fn identity<T>(v: T) -> T {
    v
}

/*
Instructions

Create a function named identity which calculates the identity of a value (receives any data type and returns the same value).
Expected Function (signature to be completed)

pub fn identity(v: _) -> _ {
}

Usage

Here is a program to test your function.

use generics::*;

fn main() {
	println!("{}", identity("Hello, world!"));
	println!("{}", identity(3));
}

And its output:

$ cargo run
Hello, world!
3
$

Notions

    Generics


*/