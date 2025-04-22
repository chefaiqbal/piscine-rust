use std::collections::HashMap;

pub fn score(input: &str) -> u64 {
    let letter_values: HashMap<char, u64> = [
        ('A', 1), ('E', 1), ('I', 1), ('O', 1), ('U', 1), ('L', 1), ('N', 1), ('R', 1), ('S', 1), ('T', 1),
        ('D', 2), ('G', 2),
        ('B', 3), ('C', 3), ('M', 3), ('P', 3),
        ('F', 4), ('H', 4), ('V', 4), ('W', 4), ('Y', 4),
        ('K', 5),
        ('J', 8), ('X', 8),
        ('Q', 10), ('Z', 10),
    ]
    .iter()
    .cloned()
    .collect();

    input
        .to_uppercase() 
        .chars() 
        .filter_map(|c| letter_values.get(&c)) 
        .sum() 


/*
Instructions

Lets play a little.

Create a function named score that given a string, computes the score for that given string as a u64.

Each letter has a value, you just have to sum the values of the letters in the given string.

You will need these:
Letter 	Value
A, E, I, O, U, L, N, R, S, T 	1
D, G 	2
B, C, M, P 	3
F, H, V, W, Y 	4
K 	5
J, X 	8
Q, Z 	10
Expected functions

    You'll need to work out the function signature for yourself.

Usage

Here is a program to test your function.

use scores::*;

fn main() {
    println!("{}", score("a"));
    println!("{}", score("ã ê Á?"));
    println!("{}", score("ThiS is A Test"));
}

And its output

$ cargo run
1
0
14
$

Notions

    patterns


*/