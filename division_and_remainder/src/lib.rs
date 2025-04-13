pub fn divide(x: i32, y: i32) -> (i32, i32) {
    (x/y,x%y)
}



/*
Create a function named divide that receives two i32 and returns a tuple. The first element is the result of the integer division between the two numbers, and the second is the remainder of the division.

pub fn divide(x: i32, y: i32) -> (i32, i32) {
}

Usage

Here is a program to test your function

use division_and_remainder::*;

fn main() {
    let x = 9;
    let y = 4;
    let (division, remainder) = divide(x, y);
    println!(
        "{}/{}: division = {}, remainder = {}",
        x, y, division, remainder
    );
}

And its output

$ cargo run
9/4: division = 2, remainder = 1
$

*/