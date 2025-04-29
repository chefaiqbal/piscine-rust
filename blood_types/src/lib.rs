use std::cmp::{Ord, Ordering};
use std::str::FromStr;  
use std::fmt::{self, Debug};

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}


impl FromStr for Antigen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err(format!("Invalid antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("Invalid Rh factor: {}", s)),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare antigens
        let antigen_ordering = self.antigen.cmp(&other.antigen);
        if antigen_ordering != Ordering::Equal {
            return antigen_ordering;
        }
        // If antigens are equal, compare Rh factors
        self.rh_factor.cmp(&other.rh_factor)
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err("Blood type string is too short".to_string());
        }
        
        let antigen_str = if s.starts_with("AB") {
            "AB"
        } else {
            &s[0..1]
        };
        
        let rh_str = if s.starts_with("AB") {
            &s[2..]
        } else {
            &s[1..]
        };
        
        let antigen = antigen_str.parse()?;
        let rh_factor = rh_str.parse()?;
        
        Ok(BloodType { antigen, rh_factor })
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        
        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        // Check antigen compatibility
        let antigen_compatible = match self.antigen {
            Antigen::O => other.antigen == Antigen::O,
            Antigen::A => other.antigen == Antigen::A || other.antigen == Antigen::O,
            Antigen::B => other.antigen == Antigen::B || other.antigen == Antigen::O,
            Antigen::AB => true, // AB can receive any antigen
        };
        
        // Check Rh factor compatibility
        let rh_compatible = match self.rh_factor {
            RhFactor::Positive => true, // Positive can receive both + and -
            RhFactor::Negative => other.rh_factor == RhFactor::Negative, // Negative can only receive negative
        };
        
        antigen_compatible && rh_compatible
    }

    pub fn donors(&self) -> Vec<Self> {
        let all_blood_types = [
            "O-", "O+", "A-", "A+", "B-", "B+", "AB-", "AB+"
        ];
        
        all_blood_types.iter()
            .filter_map(|&bt_str| {
                let bt: BloodType = bt_str.parse().unwrap();
                if self.can_receive_from(&bt) {
                    Some(bt)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let all_blood_types = [
            "O-", "O+", "A-", "A+", "B-", "B+", "AB-", "AB+"
        ];
        
        all_blood_types.iter()
            .filter_map(|&bt_str| {
                let bt: BloodType = bt_str.parse().unwrap();
                if bt.can_receive_from(self) {
                    Some(bt)
                } else {
                    None
                }
            })
            .collect()
    }
}
/*
Instructions

In this exercise you will create a data model of blood types and an API to deal with them.

First, we'll implement the data representation of the blood types.

Take a look at the BloodType struct below. It contains two enums which enable us to describe a blood type (e.g. "A-").

    Antigen: which can be one of:
        A
        B
        AB
        O
    RhFactor: which can be one of:
        Positive
        Negative

To provide a simple way to create blood types, implement the trait FromStr for BloodType. This will allow us to use the parse method and from_str, so we can do:

   let a_neg: BloodType = "A-".parse();

Implement the following Traits:

    std::cmp::Ord: to make possible to sort a vector or array of BloodType.
    std::Debug: for BloodType, allowing us print a vector such as [BloodType { antigen: A, rh_factor: Positive}, BloodType{ antigen: B, rh_factor: Negative}] as "[ A+, B-]" when using format strings {:?}.

Create three methods for BloodType:

    can_receive_from: which returns true if self can receive blood from other blood type.
    donors: which returns all the blood types that can give blood to self.
    recipients: which returns all the blood types that can receive blood from self.

Expected Functions and Structures

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
}

impl FromStr for RhFactor {
}

impl Ord for BloodType {
}

impl FromStr for BloodType {
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
	}

	pub fn donors(&self) -> Vec<Self> {
	}

	pub fn recipients(&self) -> Vec<BloodType> {
}

Usage

Here is a program to test your function.

use blood_types::*;

fn main() {
	let blood_type: BloodType = "O+".parse().unwrap();
	println!("recipients of O+ {:?}", blood_type.recipients());
	println!("donors of O+ {:?}", blood_type.donors());
	let another_blood_type: BloodType = "A-".parse().unwrap();
	println!(
		"donors of O+ can receive from {:?} {:?}",
		&another_blood_type,
		blood_type.can_receive_from(&another_blood_type)
	);
}

And its output

$ cargo run
recipients of O+ [AB+, O+, A+, B+]
donors of O+ [O+, O-]
donors of O+ can receive from A- false
$

*/