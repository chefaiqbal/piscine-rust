pub fn fibonacci(n: u32) -> u32 {

    if n <=0 {
        return 0
    }
    if n == 1 {
        return 1
    }
    else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}





/*
Instructions
Complete the body of the function fibonacci.

pub fn fibonacci(n: u32) -> u32 {
}
This function receives a number n and returns the nth number in the fibonacci series.

The Fibonacci Series starts like this: 0, 1, 1, 2, 3, 5, 8, 13 etc...

Usage
Here is a possible test for your function:

use fibonacci2::*;

fn main() {
    println!(
        "The element in the position {} in fibonacci series is {}",
        2,
        fibonacci(2)
    );
    println!(
        "The element in the position {} in fibonacci series is {}",
        4,
        fibonacci(4)
    );
    println!(
        "The element in the position {} in fibonacci series is {}",
        22,
        fibonacci(22)
    );
    println!(
        "The element in the position {} in fibonacci series is {}",
        20,
        fibonacci(20)
    );
}
And its output:

$ cargo run
The element in the position 2 in fibonacci series is 1
The element in the position 4 in fibonacci series is 3
The element in the position 22 in fibonacci series is 17711
The element in the position 20 in fibonacci series is 6765
*/