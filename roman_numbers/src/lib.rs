use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

#[derive(Clone)]
enum RomanValue {
    Single(RomanDigit),
    Pair(RomanDigit, RomanDigit),
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        let mut digits = Vec::new();
        
       
        let values: Vec<(u32, Rc<RomanValue>)> = vec![
            (1000, Rc::new(RomanValue::Single(RomanDigit::M))),
            (900, Rc::new(RomanValue::Pair(RomanDigit::C, RomanDigit::M))),
            (500, Rc::new(RomanValue::Single(RomanDigit::D))),
            (400, Rc::new(RomanValue::Pair(RomanDigit::C, RomanDigit::D))),
            (100, Rc::new(RomanValue::Single(RomanDigit::C))),
            (90, Rc::new(RomanValue::Pair(RomanDigit::X, RomanDigit::C))),
            (50, Rc::new(RomanValue::Single(RomanDigit::L))),
            (40, Rc::new(RomanValue::Pair(RomanDigit::X, RomanDigit::L))),
            (10, Rc::new(RomanValue::Single(RomanDigit::X))),
            (9, Rc::new(RomanValue::Pair(RomanDigit::I, RomanDigit::X))),
            (5, Rc::new(RomanValue::Single(RomanDigit::V))),
            (4, Rc::new(RomanValue::Pair(RomanDigit::I, RomanDigit::V))),
            (1, Rc::new(RomanValue::Single(RomanDigit::I))),
        ];

   
        for (value, roman_value_rc) in values {
            while num >= value {
                num -= value;
                match &*roman_value_rc {
                    RomanValue::Single(digit) => digits.push(*digit),
                    RomanValue::Pair(digit1, digit2) => {
                        digits.push(*digit1);
                        digits.push(*digit2);
                    },
                }
            }
        }

  
        if digits.is_empty() {
            digits.push(RomanDigit::Nulla);
        }

        RomanNumber(digits)
    }
}