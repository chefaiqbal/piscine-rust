use json::JsonValue;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        // Extract numeric value from the "calories" tuple (e.g., "510kcal")
        let kcal: f64 = food.calories.1.trim_end_matches("kcal").parse().unwrap();

        // Accumulate totals, considering the number of portions
        total_cals += kcal * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    // Helper function to round to two decimal places
    let round = |value: f64| (value * 100.0).round() / 100.0;

    // Construct the JSON object
    let mut result = JsonValue::new_object();
    result["cals"] = round(total_cals).into();
    result["carbs"] = round(total_carbs).into();
    result["proteins"] = round(total_proteins).into();
    result["fats"] = round(total_fats).into();

    result
}


/*
Instructions

Create a function named calculate_macros which receives a vector of Food structures and returns a json::JsonValue.

The Food structure has the following fields:

Food {
    name: <name>,
    calories: (<calories_in_kJ>kJ, <calories_in_kcal>kcal),
    fats: <fats_in_grams>,
    carbs: <carbs_in_grams>,
    proteins: <proteins_in_grams>,
    nbr_of_portions: <portions>
}

The name and the values in the calories tuple should be of type String. All other values should be represented as f64.

The json returned by calculate_macros will have the following format:

{
    "cals": <calories_in_kcal>,
    "carbs": <carbs_in_grams>,
    "proteins": <proteins_in_grams>,
    "fats": <fats_in_grams>,
}

Consider the number of portions, as the values of the macros refer to one portion. Each value should represent the sum of each micro-nutrient in the array. E.g. cals is the sum of all calories of all the foods combined. Every f64 should be rounded to two decimal points or one decimal point if it ends in a zero. E.g:

    12.294 -> 12.29
    12.295 -> 12.30 -> 12.3

Expected Function

pub struct Food {
    // expected public fields
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    todo!()
}

Usage

Here is a program to test your function:

use macro_calculator::*;

fn main() {
    let foods = [
        Food {
            name: "big mac".to_owned(),
            calories: ("2133.84kJ".to_owned(), "510kcal".to_owned()),
            proteins: 27.,
            fats: 26.,
            carbs: 41.,
            nbr_of_portions: 2.,
        },
        Food {
            name: "pizza margherita".to_owned(),
            calories: ("1500.59kJ".to_owned(), "358.65kcal".to_owned()),
            proteins: 13.89,
            fats: 11.21,
            carbs: 49.07,
            nbr_of_portions: 4.9,
        },
    ];

    println!("{:#}", calculate_macros(&foods));
}

And its output:

$ cargo run
{
    "cals": 2777.39,
    "carbs": 322.44,
    "proteins": 122.06,
    "fats": 106.93
}
$

*/