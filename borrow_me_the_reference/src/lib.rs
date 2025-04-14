pub fn delete_and_backspace(s: &mut String) {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = chars.len() ;	
	while i  > 0 {
		i -= 1;
        if chars[i] == '+' {
			chars.remove(i); 
            if i < chars.len() {
                chars.remove(i);
            }
        }
    }	
    i = 0;
    while i < chars.len() {
        if chars[i] == '-' {
            if i > 0 {
                chars.remove(i - 1);
                i -= 1;
            }
            chars.remove(i); 
        }  else {
            i += 1;
        }
    }
	
    *s = chars.into_iter().collect();
}



pub fn do_operations(v: &mut [String]) {
    for equation in v.iter_mut() {
        let mut left = String::new();
        let mut right = String::new();
        let mut operator = None;

        // Split the equation into left, operator, and right parts
        for c in equation.chars() {
            if c == '+' || c == '-' {
                operator = Some(c); // Store the operator
            } else if operator.is_none() {
                left.push(c); // Add to the left part
            } else {
                right.push(c); // Add to the right part
            }
        }

        // Parse the left and right parts into integers
        let left: i32 = left.parse().unwrap();
        let right: i32 = right.parse().unwrap();

        // Perform the operation based on the operator
        let result = match operator {
            Some('+') => left + right,
            Some('-') => left - right,
            _ => panic!("Unexpected operator"),
        };

        // Replace the equation with the result as a string
        *equation = result.to_string();
    }
}

/*
Instructions

    Ownership is arguably Rust's most unique feature. It enables Rust to make memory safety guarantees without needing a garbage collector.

Understanding ownership is essential to take full advantage of Rust's capabilities, as it influences almost all aspects of the language.

Create the following functions:

    delete_and_backspace: which receives a borrowed string, and processes it. - represents the backspace key and + represents the delete key, so that "helll-o" and "he+lllo" are both converted to "hello". The - and + characters should be removed from the string.

    do_operations: which borrows a vector of string literals representing simple addition and subtraction equations. The function should replace the operation with the result.

Expected Functions

pub fn delete_and_backspace(s: &mut String) {
}

pub fn do_operations(v: &mut [String]) {
}

Usage

Here is a program to test your function

use borrow_me_the_reference::*;

fn main() {
    let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
    let mut b = [
        "2+2".to_owned(),
        "3+2".to_owned(),
        "10-3".to_owned(),
        "5+5".to_owned(),
    ];

    delete_and_backspace(&mut a);
    do_operations(&mut b);

    println!("{:?}", (a, b));
}

And its output

$ cargo run
("borrow", ["4", "5", "7", "10"])
$

Notions

    Ownership

*/