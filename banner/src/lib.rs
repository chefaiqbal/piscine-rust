use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, desc: &str) -> Self {
        Self {
            short_hand: format!("-{}", &name[0..1]),
            long_hand: format!("--{}", name),
            desc: desc.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand.clone(), func);
        self.flags.insert(flag.long_hand.clone(), func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        self.flags
            .get(input)
            .ok_or_else(|| "Flag not found".to_string())
            .and_then(|callback| {
                if argv.len() < 2 {
                    Err("Not enough arguments".to_string())
                } else {
                    callback(argv[0], argv[1]).map_err(|e| e.to_string())
                }
            })
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: f64 = a.parse()?;
    let b: f64 = b.parse()?;
    Ok((a / b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: f64 = a.parse()?;
    let b: f64 = b.parse()?;
    Ok((a % b).to_string())
}
/*
Instructions
"Result is a version of the Option type that describes a possible Err instead of None".

Create a structure called Flag which has the following elements:

short_hand: String
long_hand: String
desc: String
This structure must have an associated function called opt_flag which initializes the structure. This function receives two string references and returns a structure Flag. Here is an example of its usage:

let d = Flag::opt_flag("diff", "gives the difference between two numbers");

println!("short hand: {}, long hand: {}, description: {}", d.short_hand, d.long_hand, d.desc);
// output: "short hand: -d, long hand: --diff, description: gives the difference between two numbers"
An associated type called Callback will also be provided. It should represent a function pointer which is going to be used in the structure and functions below. This function will represent the callback for the flag associated to it.

A second structure named FlagsHandler will be given which just has one element: flags: HashMap<(String, String), Callback>. You'll also need to implement the following associated functions:

add_flag, which adds the flag and callback function to the HashMap.
exec_func, which executes the function using the flag provided and returns the result. The callback should be executed with the first two arguments of the supplied argv argument. Return either the successful result from the callback or the error stringified.
You will have to create the following callback functions:

div: which converts the reference strings to f64s and returns the Result, as the division of these floats or the error ParseFloatError.
rem: which converts the reference strings to f64s and returns the Result, as the remainder of the division of these floats or the error ParseFloatError.
Expected Function
use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    // expected public fields
}

impl<'a> Flag<'a> {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        todo!()
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        todo!()
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        todo!()
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    todo!()
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    todo!()
}

Usage
Here is a program to test your function:

use banner::*;
use std::collections::HashMap;

fn main() {
    let mut handler = FlagsHandler { flags: HashMap::new() };

    let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
    let r = Flag::opt_flag(
        "remainder",
        "remainder of the division between two values, formula (a % b)",
    );

    handler.add_flag(d, div);
    handler.add_flag(r, rem);

    println!("{:?}", handler.exec_func("-d", &["1.0", "2.0"]));

    println!("{:?}", handler.exec_func("-r", &["2.0", "2.0"]));

    println!("{:?}", handler.exec_func("--division", &["a", "2.0"]));

    println!("{:?}", handler.exec_func("--remainder", &["2.0", "fd"]));
}
And its output:

$ cargo run
Ok("0.5")
Ok("0")
Err("invalid float literal")
Err("invalid float literal")
$
*/