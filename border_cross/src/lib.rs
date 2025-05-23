#[allow(dead_code)]

pub struct Car<'a> {
	pub plate_nbr: &'a str,
	pub model: &'a str,
	pub horse_power: u32,
	pub year: u32,
}

#[allow(dead_code)]
pub struct Truck<'a> {
	pub plate_nbr: &'a str,
	pub model: &'a str,
	pub horse_power: u32,
	pub year: u32,
	pub load_tons: u32,
}

pub trait Vehicle {
	 fn model(&self) -> &str;
	 fn year(&self) -> u32;
}

impl<'a> Vehicle for Truck<'a> {
    fn model(&self) -> &str {
        self.model
    }
    
    fn year(&self) -> u32 {
        self.year
    }
}


impl<'a> Vehicle for Car<'a> {
    fn model(&self) -> &str {
        self.model
    }
    
    fn year(&self) -> u32 {
        self.year
    }
}


pub fn all_models(list: Vec<&dyn Vehicle>) -> Vec<&str> {
    list.iter()
        .map(|vehicle| vehicle.model())
        .collect()
}


/*
Instructions
At a border crossing, you want to keep a list of all the vehicles that are waiting to enter the country. You intend to keep a waiting list of the vehicles, but the vehicles can be one of two types: Car or Truck.

Create a function that receives a vector of structures that implements the Vehicle trait, and returns all the models.

With the following structure:

Expected Functions and Structures
#[allow(dead_code)]
pub struct Car<'a> {
	pub plate_nbr: &'a str,
	pub model: &'a str,
	pub horse_power: u32,
	pub year: u32,
}

#[allow(dead_code)]
pub struct Truck<'a> {
	pub plate_nbr: &'a str,
	pub model: &'a str,
	pub horse_power: u32,
	pub year: u32,
	pub load_tons: u32,
}

pub trait Vehicle {
	fn model(&self) -> &str;
	fn year(&self) -> u32;
}

impl Vehicle for Truck<'_> {
}

impl Vehicle for Car<'_> {
}

fn all_models(list: Vec<&Vehicle>) -> Vec<&str> {
}
Usage
Here is a program to test your function.

fn main() {
	let vehicles: Vec</*there is something missing here*/Vehicle> = vec![
		&Car {
			plate_nbr: "A3D5C7",
			model: "Model 3",
			horse_power: 325,
			year: 2010,
		},
		&Truck {
			plate_nbr: "V3D5CT",
			model: "Ranger",
			horse_power: 325,
			year: 2010,
			load_tons: 40,
		},
	];
	println!("{:?}", all_models(vehicles));
}
And its output

$ cargo run
["Model 3", "Ranger"]
$
*/