pub fn get_diamond(c: char) -> Vec<String> {
    let size = (c as u8 - b'A') as usize; 
    let mut diamond = Vec::new();

    
    for i in 0..=size {
        let letter = (b'A' + i as u8) as char;
        let outer_spaces = size - i;
        let inner_spaces = if i == 0 { 0 } else { 2 * i - 1 };

        let mut row = String::new();
        row.push_str(&" ".repeat(outer_spaces));
        row.push(letter);
        if inner_spaces > 0 {
            row.push_str(&" ".repeat(inner_spaces));
            row.push(letter);
        }
        row.push_str(&" ".repeat(outer_spaces));
        diamond.push(row);
    }

    
    for i in (0..size).rev() {
        diamond.push(diamond[i].clone());
    }

    diamond
}

/*
Instructions
Build the function make_diamond which takes a letter as an input, and returns a diamond.

Rules:

The first and last row contain one 'A'.
The given letter has to be at the widest point.
All rows, except the first and last, have exactly two identical letters.
All rows have as many trailing spaces as leading spaces. This may be 0.
The diamond is vertically symmetrical, and horizontally symmetrical.
The width of the diamond equals its height.
The top half has letters in ascending order (abcd).
The bottom half has letters in descending order (dcba).
Expected functions
pub fn get_diamond(c: char) -> Vec<String> {

}
Usage
Here is a program to test your function.

use diamond_creation::*;

fn main() {
    println!("{:?}", get_diamond('A'));
    println!("{:?}", get_diamond('C'));
    for line in get_diamond('C') {
        println!("{}", line);
    }
}
And its output:

$ cargo run
["A"]
["  A  ", " B B ", "C   C", " B B ", "  A  "]
  A  
 B B 
C   C
 B B 
  A  
$
Notions
pattern syntax
*/