pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp_c = (c as f64).exp(); // Calculate e^c
    let ln_c = if c == 0 {
        f64::NEG_INFINITY
    } else {
        (c.abs() as f64).ln()
    };
    (c, exp_c, ln_c)
}

pub fn str_function(a: String) -> (String, String) {
    let result = a
    .split_whitespace() 
    .map(|num| {
        let n: f64 = num.parse().unwrap(); 
        n.exp().to_string() 
    })
    .collect::<Vec<String>>() 
    .join(" "); 
(a, result)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let result = b
    .iter() 
    .map(|&num| (num.abs() as f64).ln()) 
    .collect::<Vec<f64>>(); 
(b, result)
}


/*
reate the following functions. The objective is to know how ownership works with different types.

    nbr_function returns a tuple:
        with the original value.
        the exponential function of the value.
        and the natural logarithm of the absolute value.
    str_function returns a tuple:
        with the original value.
        and the exponential function of each value as a string (see the example).
    vec_function returns a tuple:
        with the original value.
        and the natural logarithm of each absolute value.

Expected functions

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
}

pub fn str_function(a: String) -> (String, String) {
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
}

Usage

Here is a possible program to test your function:

use copy::*;

fn main() {
    let a = "1 2 4 5 6".to_owned();
    let b = vec![1, 2, 4, 5];
    let c = 0;

    println!("{:?}", nbr_function(c));
    println!("{:?}", vec_function(b));
    println!("{:?}", str_function(a));
}

And its output:

$ cargo run
(0, 1.0, -inf)
([1, 2, 4, 5], [0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003])
("1 2 4 5 6", "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351")
$

Notions

    scope
    Primitive Type f64

*/