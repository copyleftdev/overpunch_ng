use crate::error::Error;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Sign {
    Positive,
    Negative,
}

pub trait Encoding: Debug + Send + Sync + 'static {
    fn encode(&self, digit: u8, sign: Sign) -> Result<char, Error>;
    fn decode(&self, c: char) -> Result<(u8, Sign), Error>;
    fn decode_digit(&self, c: char) -> Result<u8, Error>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Ebcdic;

impl Encoding for Ebcdic {
    fn encode(&self, digit: u8, sign: Sign) -> Result<char, Error> {
        if digit > 9 {
            return Err(Error::UnsupportedCharacter(digit as char));
        }

        match (digit, sign) {
            (0, Sign::Positive) => Ok('{'),
            (0, Sign::Negative) => Ok('}'),
            (1, Sign::Positive) => Ok('A'),
            (1, Sign::Negative) => Ok('J'),
            (2, Sign::Positive) => Ok('B'),
            (2, Sign::Negative) => Ok('K'),
            (3, Sign::Positive) => Ok('C'),
            (3, Sign::Negative) => Ok('L'),
            (4, Sign::Positive) => Ok('D'),
            (4, Sign::Negative) => Ok('M'),
            (5, Sign::Positive) => Ok('E'),
            (5, Sign::Negative) => Ok('N'),
            (6, Sign::Positive) => Ok('F'),
            (6, Sign::Negative) => Ok('O'),
            (7, Sign::Positive) => Ok('G'),
            (7, Sign::Negative) => Ok('P'),
            (8, Sign::Positive) => Ok('H'),
            (8, Sign::Negative) => Ok('Q'),
            (9, Sign::Positive) => Ok('I'),
            (9, Sign::Negative) => Ok('R'),
            _ => Err(Error::UnsupportedCharacter(digit as char)),
        }
    }

    fn decode(&self, c: char) -> Result<(u8, Sign), Error> {
        match c {
            '{' => Ok((0, Sign::Positive)),
            'A' => Ok((1, Sign::Positive)),
            'B' => Ok((2, Sign::Positive)),
            'C' => Ok((3, Sign::Positive)),
            'D' => Ok((4, Sign::Positive)),
            'E' => Ok((5, Sign::Positive)),
            'F' => Ok((6, Sign::Positive)),
            'G' => Ok((7, Sign::Positive)),
            'H' => Ok((8, Sign::Positive)),
            'I' => Ok((9, Sign::Positive)),
            '}' => Ok((0, Sign::Negative)),
            'J' => Ok((1, Sign::Negative)),
            'K' => Ok((2, Sign::Negative)),
            'L' => Ok((3, Sign::Negative)),
            'M' => Ok((4, Sign::Negative)),
            'N' => Ok((5, Sign::Negative)),
            'O' => Ok((6, Sign::Negative)),
            'P' => Ok((7, Sign::Negative)),
            'Q' => Ok((8, Sign::Negative)),
            'R' => Ok((9, Sign::Negative)),
            d @ '0'..='9' => Ok((d.to_digit(10).unwrap_or(0) as u8, Sign::Positive)),
            _ => Err(Error::UnsupportedCharacter(c)),
        }
    }

    fn decode_digit(&self, c: char) -> Result<u8, Error> {
        match c {
            '0'..='9' => Ok(c.to_digit(10).unwrap_or(0) as u8),
            _ => Err(Error::UnsupportedCharacter(c)),
        }
    }
}
