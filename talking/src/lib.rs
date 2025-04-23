pub fn talking(text: &str) -> &str {
  
    let trimmed = text.trim();


    if trimmed.is_empty() {
        return "Just say something!";
    }

  
    let is_yelling = trimmed.chars().any(|c| c.is_alphabetic())
        && trimmed.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());

   
    let is_question = trimmed.ends_with('?');

  
    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!", 
        (false, true) => "Sure.", 
        _ => "Interesting", 
    }
}
/*
Instructions
Build the function talking which will allow you to talk with your computer.

Its answers will be created by you following the rules below.

It answers "There is no need to yell, calm down!" if you yell at it. For example "LEAVE ME ALONE!". Yelling is when all the letters are capital letters.
It answers "Sure." if you ask it something without yelling. For example "Is everything ok with you?".
It answers "Quiet, I am thinking!" if you yell a question at it. FOr example: "HOW ARE YOU?".
It says "Just say something!" if you address it without actually saying anything.
It answers "Interesting" to anything else.
Expected functions
pub fn talking(text: &str) -> &str {

}
Usage
Here is a program to test your function.

use talking::*;

fn main() {
    println!("{:?}", talking("JUST DO IT!"));
    println!("{:?}", talking("Hello how are you?"));
    println!("{:?}", talking("WHAT'S GOING ON?"));
    println!("{:?}", talking("something"));
    println!("{:?}", talking(""));
}
And its output:

$ cargo run
"There is no need to yell, calm down!"
"Sure."
"Quiet, I am thinking!"
"Interesting"
"Just say something!"
$
Notions
patterns
*/