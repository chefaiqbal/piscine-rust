use rand::Rng; 

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Suit {
    pub fn random() -> Suit {
        Suit::Club
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid value for Suit!"), 
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        Rank::Ace
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid value for Rank!"), 
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace 
}


/*
Instructions

A standard deck of cards has 52 cards: 4 suits with 13 cards per suit. Represent the cards from a deck:

    Create an enum to represent the Suit.
    Implement the associated function random, which returns a random Suit (Heart, Diamond, Spade or Club).
    Create a Rank enum. For ace and face cards, it can be one of Ace, King, Queen or Jack. For the values from 2 to 10, it can have a Number value associated to a u8.
    Create an associated function to Rank called Random that returns a random Rank.
    Create a structure name Card which has the fields suit and rank.

Define:

    The associated function translate for Rank and Suit:
        For Suit, translate converts an integer value (u8) to a suit (1 -> Heart, 2 -> Diamonds, 3 -> Spade, 4 -> Club).
        For Rank, translate converts an integer value (u8) to a rank ( 1 -> Ace, 2 -> 2, .., 10 -> 10, 11 -> Jack, 12 -> Queen, 13 -> King).
    The associated function random for Rank and Suit returns a random Rank and Suit respectively.
    Finally define the function winner_card which returns true if the card passed as an argument is an ace of spades.

Expected Functions and Structures

pub enum Suit {
}

pub enum Rank {
}

impl Suit {
    pub fn random() -> Suit {
    }

    pub fn translate(value: u8) -> Suit {
    }
}

impl Rank {
    pub fn random() -> Rank {
    }

    pub fn translate(value: u8) -> Rank {
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
}

Usage

Here is a program to test your function

use card_deck::*;

fn main() {
    let your_card = Card {
        rank: Rank::random(),
        suit: Suit::random(),
    };

    println!("Your card is {:?}", your_card);

    if card_deck::winner_card(your_card) {
        println!("You are the winner!");
    }
}

And its output

$ cargo run
Your card is Card { suit: Club, rank: Ace }
$

*/