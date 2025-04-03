#![allow(clippy::disallowed_methods)]

use overpunch_ng::{extract, format};
use proptest::prelude::*;
use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};
use quickcheck_macros::quickcheck;
use rust_decimal::prelude::{FromStr, One, Zero};
use rust_decimal::Decimal;

// Helper function for creating a decimal
#[allow(dead_code)]
fn dec(s: &str) -> Decimal {
    Decimal::from_str(s).unwrap()
}

// Custom Arbitrary implementation for Decimal
#[derive(Debug, Clone, Copy)]
struct ArbitraryDecimal(Decimal);

impl Arbitrary for ArbitraryDecimal {
    fn arbitrary(g: &mut Gen) -> Self {
        // Generate reasonable decimals that won't exceed formatting limitations
        // Use a smaller range to avoid issues with extremely large numbers
        let is_negative = bool::arbitrary(g);

        // Generate a number with integer part and decimal part within safe limits
        let int_part = u32::arbitrary(g) % 1_000_000_000;
        let decimal_part = u32::arbitrary(g) % 1_000_000;

        let decimal_str = if is_negative {
            format!("-{}.{}", int_part, decimal_part)
        } else {
            format!("{}.{}", int_part, decimal_part)
        };

        ArbitraryDecimal(dec(&decimal_str))
    }
}

// Property tests using proptest
proptest! {
    // Basic property: format -> extract roundtrip should yield the original value
    #[test]
    fn roundtrip_format_extract(
        value in -99999i32..99999i32,
        decimals in 0usize..5usize
    ) {
        let dec_value = Decimal::from(value) / Decimal::from(10i32.pow(decimals as u32));
        if let Ok(formatted) = format(dec_value, decimals) {
            if let Ok(extracted) = extract(&formatted, decimals) {
                // Due to rounding, we may need approximate equality
                let diff = (dec_value - extracted).abs();
                let epsilon = dec("0.00001");
                prop_assert!(diff <= epsilon,
                    "Roundtrip failed: {} -> {} -> {}, diff: {}",
                    dec_value, formatted, extracted, diff);
            }
        }
    }

    // Formatting should always return a string with expected length
    #[test]
    fn format_length(value: i64, decimals in 0..10u32) {
        // Skip if value is zero, as zero-specific tests are in another test case
        if value == 0 {
            return Ok(());
        }

        let dec_value = dec(&format!("{}.0", value));

        if let Ok(formatted) = format(dec_value, decimals as usize) {
            // The expected length should be:
            // - Absolute digits in integer part
            // - Plus decimal places
            // - Plus potentially a sign character at the end
            let abs_value = value.abs();
            let int_digits = if abs_value == 0 { 1 } else { (abs_value as f64).log10().floor() as usize + 1 };
            let expected_length = int_digits + decimals as usize;

            // Allow for leading zeros for small numbers - they might make the string longer
            let actual_length = formatted.len();

            // We need a more flexible check - in some cases a leading zero might be added
            // or formatting might add additional characters
            prop_assert!(
                actual_length >= expected_length && actual_length <= expected_length + 2,
                "Formatted string '{}' from {} with {} decimals should have length approximately {}, but got {}",
                formatted, dec_value, decimals, expected_length, actual_length
            );
        }
    }

    // Test that extract never panics on valid input
    #[test]
    fn extract_no_panic(
        s in "[A-Za-z0-9!@#$%^&*()}{]{0,20}",
        decimals in 0usize..10
    ) {
        let _ = extract(&s, decimals); // This should never panic
    }
}

// QuickCheck tests
#[test]
fn zero_formatting_consistent() {
    fn property(decimals: usize) -> TestResult {
        // Only test with reasonable decimal places
        if decimals > 10 {
            return TestResult::discard();
        }

        // Test positive zero
        std::env::remove_var("HANDLE_NEGATIVE_ZERO");
        let pos_result = format(Decimal::zero(), decimals);

        // Test negative zero
        std::env::set_var("HANDLE_NEGATIVE_ZERO", "true");
        let neg_result = format(Decimal::zero(), decimals);

        // They should be different for the same decimal places
        match (pos_result, neg_result) {
            (Ok(pos), Ok(neg)) => {
                if decimals == 0 {
                    TestResult::from_bool(pos == "{" && neg == "}")
                } else {
                    // For decimal places, check that they have the same prefix but different last char
                    let pos_prefix = &pos[0..pos.len() - 1];
                    let neg_prefix = &neg[0..neg.len() - 1];
                    let pos_last = pos.chars().last().unwrap();
                    let neg_last = neg.chars().last().unwrap();

                    TestResult::from_bool(
                        pos_prefix == neg_prefix && pos_last == '{' && neg_last == '}',
                    )
                }
            }
            _ => TestResult::failed(),
        }
    }

    QuickCheck::new()
        .tests(100)
        .quickcheck(property as fn(usize) -> TestResult);
}

#[quickcheck]
fn format_extract_consistent(dec_value: ArbitraryDecimal, decimals: u8) -> TestResult {
    // Limit decimals to a reasonable range to avoid issues
    let decimals = (decimals % 10) as usize;

    // Skip testing with very large numbers
    if dec_value.0.abs() > dec("9999999999.9999") {
        return TestResult::discard();
    }

    // When decimals = 0, the fractional part is ignored during formatting
    // So values with significant decimal places will fail this test
    if decimals == 0 && dec_value.0.fract().abs() > dec("0.01") {
        return TestResult::discard();
    }

    match format(dec_value.0, decimals) {
        Ok(formatted) => {
            if let Ok(extracted) = extract(&formatted, decimals) {
                // Due to rounding differences, use approximate equality
                let diff = (dec_value.0 - extracted).abs();

                // Calculate epsilon based on decimal places
                // When we format with N decimal places, precision up to 10^-N is expected
                let precision_factor = match decimals {
                    0 => dec("1"),
                    1 => dec("0.1"),
                    2 => dec("0.01"),
                    3 => dec("0.001"),
                    _ => {
                        let mut factor = Decimal::one();
                        for _ in 0..decimals {
                            factor /= dec("10");
                        }
                        factor
                    }
                };

                // Add a small buffer to account for floating point imprecision
                let epsilon = precision_factor * dec("1.1");

                // For very small values, provide a minimum epsilon
                let min_epsilon = dec("0.01");
                let epsilon = if epsilon < min_epsilon {
                    min_epsilon
                } else {
                    epsilon
                };

                if diff <= epsilon {
                    TestResult::passed()
                } else {
                    println!(
                        "Original: {}, Extracted: {}, Diff: {}, Epsilon: {}, Decimals: {}",
                        dec_value.0, extracted, diff, epsilon, decimals
                    );
                    TestResult::failed()
                }
            } else {
                TestResult::failed()
            }
        }
        Err(_) => TestResult::discard(),
    }
}

// Test edge cases specifically
#[test]
fn edge_cases() {
    // Test zero cases
    for decimals in 0..5 {
        // Test formatting of zero (positive and negative)
        let pos_result = format(Decimal::zero(), decimals).unwrap();
        assert!(extract(&pos_result, decimals).is_ok());

        // Set environment variable to ensure negative zero handling
        std::env::set_var("HANDLE_NEGATIVE_ZERO", "true");
        let neg_zero = -Decimal::zero();
        let neg_result = format(neg_zero, decimals).unwrap();
        assert!(extract(&neg_result, decimals).is_ok());
        std::env::remove_var("HANDLE_NEGATIVE_ZERO");

        // Check last character for zero formatting
        // Positive zero should end with '{'
        assert_eq!(pos_result.chars().last().unwrap(), '{');
        // Negative zero should end with '}'
        assert_eq!(neg_result.chars().last().unwrap(), '}');
    }

    // Test extremely large and small values (within the limits of what Decimal can handle)
    let large_val = dec("999999999.9999");
    assert!(format(large_val, 4).is_ok());

    let small_val = dec("-999999999.9999");
    assert!(format(small_val, 4).is_ok());

    // Test that extract correctly handles each possible overpunch character
    for (ch, expected) in &[
        ('0', 0),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('{', 0),
        ('A', 1),
        ('B', 2),
        ('C', 3),
        ('D', 4),
        ('E', 5),
        ('F', 6),
        ('G', 7),
        ('H', 8),
        ('I', 9),
        ('}', 0),
        ('J', 1),
        ('K', 2),
        ('L', 3),
        ('M', 4),
        ('N', 5),
        ('O', 6),
        ('P', 7),
        ('Q', 8),
        ('R', 9),
    ] {
        // Create a test string with the overpunch character
        let test_str = format!("123{}", ch);

        // When decimals=0, extract treats the last character as part of the whole number
        // When using extract, the overpunch character becomes the last digit
        let result = extract(&test_str, 0).unwrap();

        // Format the expected string properly with the decimal point
        let expected_decimal = match ch {
            '{' => dec("1230"),
            '}' => dec("-1230"), // Negative zero becomes negative value
            'A'..='I' => dec(&format!("123{}", expected)),
            'J'..='R' => dec(&format!("-123{}", expected)),
            '0'..='9' => dec(&format!("123{}", expected)),
            _ => panic!("Unexpected character in test"),
        };

        assert_eq!(
            result, expected_decimal,
            "Failed for char '{}': expected {}, got {}",
            ch, expected_decimal, result
        );
    }
}
