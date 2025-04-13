pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val)
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    &slice[index]
}
/*
Create a function named insert, that inserts a new element at the end of the Vec.

Create another function named at_index that returns the value found at the index passed as an argument.

pub fn insert(vec: &mut Vec<String>, val: String) {
}

pub fn at_index(slice: &[String], index: usize) -> &str {
}
Usage
Here is a possible program to test your function:

use groceries::*;

fn main() {
    let mut groceries = vec![
        "yogurt".to_string(),
        "panettone".to_string(),
        "bread".to_string(),
        "cheese".to_string(),
    ];
    insert(&mut groceries, String::from("nuts"));
    println!("groceries = {:?}", &groceries);
    println!("groceries[1] = {:?}", at_index(&groceries, 1));
}
And its output:

$ cargo run
groceries = ["yogurt", "panettone", "bread", "cheese", "nuts"]
groceries[1] = "panettone"
$
*/