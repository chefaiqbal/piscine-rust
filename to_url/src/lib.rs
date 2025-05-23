pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")
}


/*
Instructions
Create a function named to_url which takes a string and substitutes every ASCII space with "%20".

Expected functions
pub fn to_url(s: &str) -> String {
}
Usage
Here is a program to test your function.

use to_url::*;

fn main() {
    let s = "Hello, world!";
    println!("'{}' parsed as an URL becomes '{}'", s, to_url(s));
}
And its output

$ cargo run
'Hello, world!' parsed as an URL becomes 'Hello,%20world!'
$

*/