pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();

    for name in names {
        let mut initials = String::new();

        for word in name.split_whitespace() {
            if let Some(first_char) = word.chars().next() {
                initials.push(first_char.to_ascii_uppercase()); 
                initials.push('.'); 
                initials.push(' '); 
            }
        }

        initials.pop();
        result.push(initials);
    }

    result
}

/* 
Instructions
Create a function named initials. This function will receive a vector of string literals with names, and return a vector of Strings with the initials of each name.

Expected Functions
pub fn initials(names: Vec<&str>) -> Vec<String> {
}
Your heap allocations will be monitored to ensure that you do not make too many allocations, and that your allocations are reasonably sized.

Usage
Here is a program to test your function:

use name_initials::*;

fn main() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
}
And its output

$ cargo run
["H. P.", "S. E.", "J. L.", "B. O."]
$
Notions
stack and heap
*/