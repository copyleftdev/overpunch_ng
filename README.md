# Overpunch_ng

A Rust library for handling overpunch encoding and decoding of decimal values. Overpunch encoding is a technique used in legacy COBOL and mainframe systems to efficiently represent signed decimal values.

## Features

- Convert decimal values to and from overpunch format
- Support for multiple encoding schemes (ASCII, EBCDIC)
- Highly optimized for performance
- Comprehensive error handling
- Property-based testing for robustness

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
overpunch_ng = "0.1.0"
```

## Usage

### Basic Examples

```rust
use overpunch_ng::{format, extract};
use rust_decimal::Decimal;
use std::str::FromStr;

// Format a decimal value with overpunch encoding
let value = Decimal::from_str("123.45").unwrap();
let formatted = format(value, 2).unwrap();
println!("Formatted: {}", formatted);  // Output: "123E5"

// Extract a decimal value from an overpunched string
let extracted = extract("123E5", 2).unwrap();
println!("Extracted: {}", extracted);  // Output: 123.45
```

### Working with Different Encodings

```rust
use overpunch_ng::{format_with_encoding, extract_with_encoding, encoding::Encoding};
use rust_decimal::Decimal;
use std::str::FromStr;

// Format using EBCDIC encoding
let value = Decimal::from_str("-123.45").unwrap();
let formatted = format_with_encoding(value, 2, &overpunch_ng::encoding::Ebcdic).unwrap();

// Extract using EBCDIC encoding
let extracted = extract_with_encoding(&formatted, 2, &overpunch_ng::encoding::Ebcdic).unwrap();
```

## Advanced Features

### Custom Encodings

You can implement your own encoding by implementing the `Encoding` trait:

```rust
use overpunch_ng::{encoding::{Encoding, Sign}, error::Error};

struct MyCustomEncoding;

impl Encoding for MyCustomEncoding {
    fn encode(&self, digit: u8, sign: Sign) -> Result<char, Error> {
        // Your implementation here
    }

    fn decode(&self, c: char) -> Result<(u8, Sign), Error> {
        // Your implementation here
    }
    
    fn decode_digit(&self, c: char) -> Result<u8, Error> {
        // Your implementation here
    }
}

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Maintained by [copyleftdev](https://github.com/copyleftdev).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.