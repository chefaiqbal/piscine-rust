pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let below_20 = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    let thousands = ["", "thousand", "million"];

    let mut num = n;
    let mut words = Vec::new();
    let mut thousand_index = 0;

    while num > 0 {
        if num % 1000 != 0 {
            words.insert(
                0,
                format!(
                    "{} {}",
                    spell_below_1000((num % 1000) as u16, &below_20, &tens),
                    thousands[thousand_index]
                )
                .trim()
                .to_string(),
            );
        }
        num /= 1000;
        thousand_index += 1;
    }

    words.join(" ").trim().to_string()
}

fn spell_below_1000(n: u16, below_20: &[&str], tens: &[&str]) -> String {
    let mut result = String::new();

    if n >= 100 {
        result.push_str(below_20[(n / 100) as usize]);
        result.push_str(" hundred");
        if n % 100 != 0 {
            result.push(' ');
        }
    }

    let remainder = n % 100;
    if remainder < 20 {
        result.push_str(below_20[remainder as usize]);
    } else {
        result.push_str(tens[(remainder / 10) as usize]);
        if remainder % 10 != 0 {
            result.push('-');
            result.push_str(below_20[(remainder % 10) as usize]);
        }
    }

    result
}

/*
Instructions
In this exercise, you'll create the function spell that will spell a generated number.

Here are some examples of what your function should return:

1 -> "one"
14 -> "fourteen".
96 -> "ninety-six"
100 -> "one hundred".
101 -> "one hundred one"
348 -> "three hundred forty-eight"
1002 -> "one thousand two".
1000000 -> "one million"
Only positive numbers will be tested, up to "one million".

Expected function
pub fn spell(n: u64) -> String {

}
Usage
Here is a program to test your function.

use spelling::*;

fn main() {
    println!("{}", spell(348));
    println!("{}", spell(9996));
}
And its output:

$ cargo run
three hundred forty-eight
nine thousand nine hundred ninety-six
$
Notions
patterns
*/