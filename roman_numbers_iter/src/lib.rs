use crate::RomanDigit::*;

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

#[derive(Debug, Clone, PartialEq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            1..=4 => I,
            5..=9 => V,
            10..=49 => X,
            50..=99 => L,
            100..=499 => C,
            500..=999 => D,
            1000..=5000 => M,
            _ => Nulla,
        }
    }
}

impl From<RomanDigit> for u32 {
    fn from(n: RomanDigit) -> Self {
        match n {
            I => 1,
            V => 5,
            X => 10,
            L => 50,
            C => 100,
            D => 500,
            M => 1000,
            _ => 0,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(n: u32) -> Self {
        if n == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut quotient = n;
        let mut p = 0;
        let mut reverse_roman = Vec::new();

        while quotient != 0 {
            let rest = quotient % 10;
            quotient /= 10;
            p += 1;
            
            match rest {
                9 => {
                    reverse_roman.push(RomanDigit::from(10_u32.pow(p)));
                    reverse_roman.push(RomanDigit::from(10_u32.pow(p - 1)));
                }
                4 => {
                    reverse_roman.push(RomanDigit::from(10_u32.pow(p) / 2));
                    reverse_roman.push(RomanDigit::from(10_u32.pow(p - 1)));
                }
                5..=8 => {
                    reverse_roman.extend(
                        std::iter::repeat(RomanDigit::from(10_u32.pow(p - 1)))
                            .take((rest - 5) as usize)
                    );
                    reverse_roman.push(RomanDigit::from(10_u32.pow(p) / 2));
                }
                1..=3 => {
                    reverse_roman.extend(
                        std::iter::repeat(RomanDigit::from(10_u32.pow(p - 1)))
                            .take(rest as usize)
                    );
                }
                _ => {}
            }
        }

        reverse_roman.reverse();
        RomanNumber(reverse_roman)
    }
}

impl From<RomanNumber> for u32 {
    fn from(n: RomanNumber) -> Self {
        let numbers: Vec<u32> = n.0.iter()
            .map(|&digit| u32::from(digit))
            .collect();
            
        numbers.iter()
            .enumerate()
            .fold(0, |acc, (i, &value)| {
                if i > 0 && value > numbers[i - 1] {
                    acc + value - 2 * numbers[i - 1]
                } else {
                    acc + value
                }
            })
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        let next_value = u32::from(self.clone()) + 1;
        Some(RomanNumber::from(next_value))
    }
}