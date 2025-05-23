use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
	Top,
	Bottom,
	Center,
}

#[derive(Debug, Eq, PartialEq)]
 pub struct Notification {
	pub size: u32,
	pub color: (u8, u8, u8),
	pub position: Position,
	pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
	Remainder(&'a str),
	Registration(Duration),
	Appointment(&'a str),
	Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format: (Position, size, colored_content)
        write!(
            f,
            "({:?}, {}, {})",
            self.position,
            self.size,
            self.content.truecolor(self.color.0, self.color.1, self.color.2)
        )
    }
}

use Event::*;

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            
            Registration(duration) => {
                // Convert duration to hours, minutes, seconds
                let seconds = duration.num_seconds();
                let hours = seconds / 3600;
                let minutes = (seconds % 3600) / 60;
                let secs = seconds % 60;
                
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!("You have {}H:{}M:{}S left before the registration ends", 
                                    hours, minutes, secs),
                }
            },
            
            Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
            },
            
            Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}

/*
Instructions
You have to design a notification system for a platform.

Depending on the type of event, your event handler will control the size, color and position of the notification.

Create a method named notify which returns a Notification with the following characteristics for each of:

Remainder(text):
size: 50
color: (50, 50, 50)
position: Bottom
content: the text associated to the enum variant
Registration(chrono::Duration):
size: 30
color: (255, 2, 22)
position: Top
content: "You have {duration} left before the registration ends"
Appointment(text):
size: 100
color: (200, 200, 3)
position: Center
content: text associated to the value
Holiday:
size: 25
color: (0, 255, 0)
position: Top
content: "Enjoy your holiday"
duration must be displayed in the form of {hours}H:{minutes}M:{seconds}S. The time will represent the remaining time before the event starts. For example, if there are 13 hours, 38 minutes and 14 seconds left, then the content will be "You have 13H:38M:14S left before the registration ends"

Implement the std::fmt::Display trait so the text of the notifications are printed in the right color in the command line.

Dependencies
chrono = "0.4"

colored = "2.0.0"

Expected Functions and Data Structures
use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
	Top,
	Bottom,
	Center,
}

#[derive(Debug, Eq, PartialEq)]
 pub struct Notification {
	pub size: u32,
	pub color: (u8, u8, u8),
	pub position: Position,
	pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
	Remainder(&'a str),
	Registration(Duration),
	Appointment(&'a str),
	Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
}

use Event::*;

impl Event {
	pub fn notify(&self) -> Notification {
	}
}
Usage
Here is a program to test your function.

use events::Event::*;
use chrono::Duration;

fn main() {
	let remainder = Remainder("Go to the doctor");
	println!("{}", remainder.notify());
	let registration = Registration(Duration::seconds(49094));
	println!("{}", registration.notify());
	let appointment = Appointment("Go to the doctor");
	println!("{}", appointment.notify());
	let holiday = Holiday;
	println!("{}", holiday.notify());
}
And its output

$ cargo run
(Bottom, 50, [38;2;50;50;50mGo to the doctor[0m)
(Top, 30, [38;2;255;2;22mYou have 13H:38M:14S left before the registration ends[0m)
(Center, 100, [38;2;200;200;3mGo to the doctor[0m)
(Top, 25, [38;2;0;255;0mEnjoy your holiday[0m)
$
Notions
colored crate
chrono crate
*/