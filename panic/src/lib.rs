use std::fs::File;

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap()
}

/*
Instructions
Create a function that tries to open a file and panics if the file does not exist.

Expected Function
pub fn open_file(s: &str) -> File {
    todo!()
}
Usage
Here is a program to test your function:

use panic::*;
use std::fs::{self, File};

fn main() {
    let filename = "created.txt";
    File::create(filename).unwrap();

    println!("{:?}", open_file(filename));

    fs::remove_file(filename).unwrap();

    // this should panic!
    open_file(filename);
}
And its output:

$ cargo run
File { fd: 3, path: ".../src/created.txt", read: true, write: false }

thread 'main' panicked at src/main.rs:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
...
$
*/