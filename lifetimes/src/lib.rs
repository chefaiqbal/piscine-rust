#[derive(Debug, Clone)]
pub struct Person<'a> {
	pub name: &'a str,
	pub age: u8,
}

impl <'a> Person<'a>{
	pub fn new(name: &'a str) -> Person<'a> {
        Person {
            name,
            age: 0,
        }
	}
}


/*
Instructions
Complete the Person struct with the fields and associated function described below. new should set the age to 0.

Expected Functions and Data Structures (Both need to be completed)
#[derive(Debug)]
pub struct Person{
	pub name: &str,
	pub age: u8,
}

impl Person {
	pub fn new(name: &str) -> Person {
	}
}
Usage
Here is a program to test your function.

use lifetimes::*;

fn main() {
	let person = Person::new("Leo");

	println!("Person = {:?}", person);
}
And its output:

$ cargo run
Person = Person { name: "Leo", age: 0 }
$
Notions
lifetimes
*/