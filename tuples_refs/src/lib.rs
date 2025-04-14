
pub struct Student(pub u32, pub String, pub String);
pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> &str {
    &student.1
}

pub fn last_name(student: &Student) -> &str {
    &student.2
}


/*
Instructions
Define a tuple struct to represent a Student. Each is identified by an id of type u32, their first name and last name.

Then define three functions to return the id, first name and last name.

pub fn id(student: &Student) -> u32 {
}

pub fn first_name(student: &Student) -> &str {
}

pub fn last_name(student: &Student) -> &str {
}
Usage
Here is a program to test your functions

use tuples_refs::*;

fn main() {
    let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
    println!("Student's first name: {}", first_name(&student));
    println!("Student's last name: {}", last_name(&student));
    println!("Student's id: {}", id(&student));
}
And its output:

$ cargo run
Student's first name: Pedro
Student's last name: Domingos
Student's id: 20
$
*/