use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
   
    let days_in_year = if NaiveDate::from_ymd_opt(year as i32, 12, 31).is_some() {
        NaiveDate::from_ymd_opt(year as i32, 12, 31).unwrap().ordinal()
    } else {
        return None; 
    };


    if days_in_year % 2 == 0 {
        return None;
    }


    let middle_day_ordinal = (days_in_year / 2) + 1;

    let middle_date = NaiveDate::from_yo_opt(year as i32, middle_day_ordinal)?;

    Some(middle_date.weekday())
}


/*
Instructions
Use the chrono crate to create a function named middle_day. It accepts a year, and returns the weekday of the middle day of that year, wrapped in an Option.

Years with an even number of days do not have an exact single middle day, and thus should return None.

Expected Function
pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    todo!()
}
Usage
Here is a program to test your function:

use middle_day::*;

fn main() {
    println!("{:?}", middle_day(1022));
}
And its output:

$ cargo run
Some(Tue)
$
Notions:
chrono-0.4.40
*/