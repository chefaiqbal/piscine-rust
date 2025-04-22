pub fn num_to_ordinal(x: u32) -> String {
    let suffix = match x % 100 {
        11 | 12 | 13=> "th", 
        _ => match x % 10 {
            1 => "st", 
            2 => "nd", 
            3 => "rd", 
            _ => "th", 
        },
    };
    format!("{}{}", x, suffix) 
}


/*Instructions

Complete the function num_to_ordinal. It returns the ordinal number for a given cardinal number.
Expected functions

pub fn num_to_ordinal(x: u32) -> String {

}

Usage

Here is a program to test your function.

use ordinal::*;

fn main() {
    println!("{}", num_to_ordinal(1));
    println!("{}", num_to_ordinal(22));
    println!("{}", num_to_ordinal(43));
    println!("{}", num_to_ordinal(47));
}

And its output

$ cargo run
1st
22nd
43rd
47th
$
*/