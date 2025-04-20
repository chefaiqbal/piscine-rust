pub struct One {
    pub first_layer: Option<Two>,
}

pub struct Two {
    pub second_layer: Option<Three>,
}

pub struct Three {
    pub third_layer: Option<Four>,
}

pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        self.first_layer?
            .second_layer?
            .third_layer?
            .fourth_layer
    }
}

/*
Instructions
Create the following structures. Each has one field:

One: first_layer with type Option<Two>.
Two: second_layer with type Option<Three>
Three: third_layer with type Option<Four>
Four: fourth_layer with type Option<u16>.
You must also create a function associated to the structure One called get_fourth_layer, which should return the fourth_layer value in the Four structure.

Expected Function and structures
pub struct One {
    // expected public fields
}
pub struct Two {
    // expected public fields
}
pub struct Three {
    // expected public fields
}
pub struct Four {
    // expected public fields
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        todo!()
    }
}
Usage
Here is a program to test your function:

use question_mark::*;

fn main() {
    let a = One {
        first_layer: Some(Two {
            second_layer: Some(Three {
                third_layer: Some(Four {
                    fourth_layer: Some(1000)
                })
            })
        })
    };

    println!("{:?}", a.get_fourth_layer());
}
And its output:

$ cargo run
Some(1000)
$
*/