pub fn search(array: &[i32], key: i32) -> Option<usize> {
    array.iter().position(|&x| x == key)
}

/*
Instructions

Complete the function search. It should return the index of the element which matches key in the array.

    Only arrays with unique elements will be tested.

Expected functions

pub fn search(array: &[i32], key: i32) -> Option<usize> {

}

Usage

Here is a program to test your function.

use searching::*;

fn main() {
    let ar = [1, 3, 4, 6, 8, 9, 11];
    let f = search(&ar, 6);
    println!(
        "the element 6 is in the position {:?} in the array {:?}",
        f, ar
    );
}

And its output:

$ cargo run
the element 6 is in the position Some(3) in the array [1, 3, 4, 6, 8, 9, 11]
$

Notions

    patterns


*/