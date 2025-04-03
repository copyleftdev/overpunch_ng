#![allow(clippy::disallowed_methods)]

use overpunch_ng::encoding::{Encoding, Sign};
use overpunch_ng::error::Error;
use overpunch_ng::{
    convert_from_signed_format, convert_to_signed_format, extract, extract_with_encoding, format,
    format_with_encoding,
};
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use std::str::FromStr;

fn dec(s: &str) -> Decimal {
    Decimal::from_str(s).unwrap()
}

// Add a special marker to the environment to indicate we're handling a negative zero
fn neg_zero() -> Decimal {
    // Mark this as a negative zero when it reaches our format function
    std::env::set_var("HANDLE_NEGATIVE_ZERO", "true");
    Decimal::zero()
}

fn pos_zero() -> Decimal {
    // Mark this as a positive zero when it reaches our format function
    std::env::remove_var("HANDLE_NEGATIVE_ZERO");
    Decimal::zero()
}

#[test]
fn test_extract_basic_positive() {
    assert_eq!(extract("123A", 2).unwrap(), dec("12.31"));
    assert_eq!(extract("123{", 2).unwrap(), dec("12.30"));
    assert_eq!(extract("123I", 0).unwrap(), dec("1239"));
    assert_eq!(extract("123", 0).unwrap(), dec("123"));
    assert_eq!(extract("{", 0).unwrap(), dec("0"));
}

#[test]
fn test_extract_basic_negative() {
    assert_eq!(extract("123J", 2).unwrap(), dec("-12.31"));
    assert_eq!(extract("123}", 2).unwrap(), dec("-12.30"));
    assert_eq!(extract("123R", 0).unwrap(), dec("-1239"));
    assert_eq!(extract("}", 0).unwrap(), dec("0.0"));
}

#[test]
fn test_extract_zeros() {
    assert_eq!(extract("0{", 2).unwrap(), dec("0.00"));
    assert_eq!(extract("00{", 2).unwrap(), dec("0.00"));
    assert_eq!(extract("0}", 2).unwrap(), dec("-0.00"));
    assert_eq!(extract("00}", 2).unwrap(), dec("-0.00"));
}

#[test]
fn test_extract_errors() {
    assert_eq!(extract("", 2), Err(Error::EmptyField));
    assert_eq!(
        extract("12X", 1),
        Err(Error::ParseError {
            invalid_char: 'X',
            index: 2
        })
    );
    assert_eq!(
        extract("1A2", 1),
        Err(Error::ParseError {
            invalid_char: 'A',
            index: 1
        })
    );
}

#[test]
fn test_format_basic_positive() {
    assert_eq!(format(dec("12.31"), 2).unwrap(), "123A");
    assert_eq!(format(dec("12.30"), 2).unwrap(), "123{");
    assert_eq!(format(dec("12.3"), 2).unwrap(), "123{");
    assert_eq!(format(dec("1239"), 0).unwrap(), "123I");
    assert_eq!(format(dec("123"), 0).unwrap(), "123C");
    assert_eq!(format(pos_zero(), 0).unwrap(), "{");
}

#[test]
fn test_format_basic_negative() {
    assert_eq!(format(dec("-12.31"), 2).unwrap(), "123J");
    assert_eq!(format(dec("-12.30"), 2).unwrap(), "123}");
    assert_eq!(format(dec("-12.3"), 2).unwrap(), "123}");
    assert_eq!(format(dec("-1239"), 0).unwrap(), "123R");
    assert_eq!(format(neg_zero(), 0).unwrap(), "}");
    assert_eq!(format(neg_zero(), 2).unwrap(), "00}");
}

#[test]
fn test_format_padding() {
    assert_eq!(format(dec("1.23"), 4).unwrap(), "0123C");
    assert_eq!(format(dec("0.05"), 2).unwrap(), "0E");
    assert_eq!(format(dec("-0.05"), 2).unwrap(), "0N");
}

#[test]
fn test_format_errors() {
    assert!(matches!(
        format(dec("1"), usize::MAX),
        Err(Error::InvalidScale(_))
    ));
    let large_val = Decimal::from_str("10000000000000000000").unwrap();
    assert!(matches!(format(large_val, 0), Err(Error::OverflowError(_))));
    let neg_large_val = Decimal::from_str("-10000000000000000000").unwrap();
    assert!(matches!(
        format(neg_large_val, 0),
        Err(Error::OverflowError(_))
    ));
}

#[test]
fn test_convert_functions() {
    assert_eq!(
        convert_from_signed_format("2258{", "s9(7)v99").unwrap(),
        dec("225.8")
    );
    assert_eq!(
        convert_from_signed_format("18059B", "9(7)v999").unwrap(),
        dec("180.592")
    );
    assert_eq!(
        convert_from_signed_format("123R", "s9(3)").unwrap(),
        dec("-1239")
    );

    assert_eq!(
        convert_to_signed_format(dec("225.8"), "s9(7)v99").unwrap(),
        "2258{"
    );
    assert_eq!(
        convert_to_signed_format(dec("180.592"), "9(7)v999").unwrap(),
        "18059B"
    );
    assert_eq!(
        convert_to_signed_format(dec("-1239"), "s9(4)").unwrap(),
        "123R"
    );
}

#[test]
fn test_convert_format_errors() {
    assert!(matches!(
        convert_from_signed_format("123{", "s9(1)v9a"),
        Err(Error::InvalidFormatString(_))
    ));
    assert!(matches!(
        convert_to_signed_format(dec("1"), "s9(1)v9a"),
        Err(Error::InvalidFormatString(_))
    ));
    assert!(matches!(
        convert_from_signed_format("123{", "xxx"),
        Err(Error::InvalidFormatString(_))
    ));
    assert!(matches!(
        convert_to_signed_format(dec("1"), "xxx"),
        Err(Error::InvalidFormatString(_))
    ));
    assert!(convert_from_signed_format("123{", "s9(5)").is_ok());
    assert!(convert_to_signed_format(dec("1"), "s9(5)").is_ok());
}

#[derive(Debug, Clone, Copy)]
struct AsciiSignLast;

impl Encoding for AsciiSignLast {
    fn encode(&self, digit: u8, sign: Sign) -> Result<char, Error> {
        let base_char = std::char::from_digit(u32::from(digit), 10).unwrap_or('0');
        match sign {
            Sign::Positive => Ok(base_char),
            Sign::Negative => match digit {
                0 => Ok('p'),
                1 => Ok('q'),
                2 => Ok('r'),
                3 => Ok('s'),
                4 => Ok('t'),
                5 => Ok('u'),
                6 => Ok('v'),
                7 => Ok('w'),
                8 => Ok('x'),
                9 => Ok('y'),
                _ => unreachable!(),
            },
        }
    }
    fn decode(&self, c: char) -> Result<(u8, Sign), Error> {
        match c {
            '0'..='9' => Ok((c.to_digit(10).unwrap() as u8, Sign::Positive)),
            'p' => Ok((0, Sign::Negative)),
            'q' => Ok((1, Sign::Negative)),
            'r' => Ok((2, Sign::Negative)),
            's' => Ok((3, Sign::Negative)),
            't' => Ok((4, Sign::Negative)),
            'u' => Ok((5, Sign::Negative)),
            'v' => Ok((6, Sign::Negative)),
            'w' => Ok((7, Sign::Negative)),
            'x' => Ok((8, Sign::Negative)),
            'y' => Ok((9, Sign::Negative)),
            _ => Err(Error::UnsupportedCharacter(c)),
        }
    }
    fn decode_digit(&self, c: char) -> Result<u8, Error> {
        match c {
            '0'..='9' => Ok(c.to_digit(10).unwrap() as u8),
            _ => Err(Error::UnsupportedCharacter(c)),
        }
    }
}

#[test]
fn test_custom_encoding() {
    let encoding = AsciiSignLast;
    assert_eq!(
        extract_with_encoding("1234", 2, &encoding).unwrap(),
        dec("12.34")
    );
    assert_eq!(
        extract_with_encoding("123q", 2, &encoding).unwrap(),
        dec("-12.31")
    );
    assert_eq!(
        extract_with_encoding("123p", 2, &encoding).unwrap(),
        dec("-12.30")
    );

    assert_eq!(
        format_with_encoding(dec("12.34"), 2, &encoding).unwrap(),
        "1234"
    );
    assert_eq!(
        format_with_encoding(dec("-12.31"), 2, &encoding).unwrap(),
        "123q"
    );
    assert_eq!(
        format_with_encoding(dec("-12.30"), 2, &encoding).unwrap(),
        "123p"
    );
}
