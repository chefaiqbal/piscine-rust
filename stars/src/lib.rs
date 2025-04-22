
pub fn stars(n: u32) -> String {
    "*".repeat(2_usize.pow(n))
}

/*
Instructions

Create a function named stars that takes a u32 as an argument and returns a String of stars (asterisks). The number of stars returned is 2^n (2 to the nth power).
Expected functions

pub fn stars(n: u32) -> String {

}

Usage

Here is a program to test your function.

use stars::stars;

fn main() {
    println!("{}", stars(1));
    println!("{}", stars(4));
    println!("{}", stars(5));
}

And its output:

$ cargo run
**
****************
********************************
$

*/