pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/*
Instructions
Create the function add_curry, which returns a closure. The purpose is to 'curry' the add method to create more variations.

Usage
Here is a program to test your function.

use adding::add_curry;

fn main() {
    let add10 = add_curry(-10);
    let add20 = add_curry(2066);
    let add30 = add_curry(300000);

    println!("{}", add10(5));
    println!("{}", add20(195));
    println!("{}", add30(5696));
}
And its output:

$ cargo run
-5
2261
305696
$
*/