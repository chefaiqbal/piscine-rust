#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected: String = original
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                if c.is_ascii_lowercase() {
                    (b'z' - (c as u8 - b'a')) as char
                } else {
                    (b'Z' - (c as u8 - b'A')) as char
                }
            } else {
                c
            }
        })
        .collect();

    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}

#[test]
fn test_ok_values() {
    assert_eq!(cipher("1Hello 2world!", "1Svool 2dliow!"), Ok(()));
    assert_eq!(cipher("asdasd", "zhwzhw"), Ok(()));
    assert_eq!(cipher("3(/&%fsd 32das", "3(/&%uhw 32wzh"), Ok(()));
}

#[test]
fn test_empty_values() {
    assert_eq!(cipher("", ""), Ok(()));
    assert_eq!(
        cipher("", "1Svool 2dliow!"),
        Err(CipherError {
            expected: "".to_owned()
        })
    );
    assert_eq!(
        cipher("1Hello 2world!", ""),
        Err(CipherError {
            expected: "1Svool 2dliow!".to_owned()
        })
    );
}

/*
Instructions
The Atbash cipher is an encryption method in which each letter of a word is replaced by its mirror letter in the alphabet.

Your objective is to create a function named cipher which must return a Result.

cipher should compare the original string with a ciphered string. It should return an empty value (()) if the cipher is correct. If the cipher is incorrect it should return the error type CipherError with the expected cipher (expected: String).

Expected Function and structure
#[derive(Debug, PartialEq)]
pub struct CipherError {
    // expected public fields
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    todo!()
}
Usage
Here is a program to test your function:

use cipher::*;

fn main() {
    println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
    println!("{:?}", cipher("1Hello 2world!", "svool"));
}
And its output:

$ cargo run
Ok(())
Err(CipherError { expected: "1Svool 2dliow!" })
$
*/