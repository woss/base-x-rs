//! # base_x
//!
//! Encode and decode any base alphabet.
//!
//! ## Installation
//!
//! Add this to `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! base-x = "0.2.0"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! fn main() {
//!   let decoded = base_x::decode("01", "11111111000000001111111100000000").unwrap();
//!   let encoded = base_x::encode("01", &decoded);
//!  assert_eq!(encoded, "11111111000000001111111100000000");
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod alphabet;
mod bigint;
pub mod decoder;
pub mod encoder;

pub use alphabet::Alphabet;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

#[cfg(not(feature = "std"))]
use core as std;

use std::fmt;

#[derive(Debug)]
pub struct DecodeError;

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to decode the given data")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

/// Encode an input vector using the given alphabet.
pub fn encode<A: Alphabet>(alphabet: A, input: &[u8]) -> String {
    alphabet.encode(input)
}

/// Decode an input vector using the given alphabet.
pub fn decode<A: Alphabet>(alphabet: A, input: &str) -> Result<Vec<u8>, DecodeError> {
    alphabet.decode(input)
}

#[cfg(test)]
mod test {
    use super::decode;
    use super::encode;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn works() {
        let mut file = File::open("./fixtures/fixtures.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let json: serde_json::Value = serde_json::from_str(&data).unwrap();
        let alphabets = &json["alphabets"];

        for value in json["valid"].as_array().unwrap() {
            let alphabet_name = value["alphabet"].as_str().unwrap();
            let input = value["string"].as_str().unwrap();
            let alphabet = alphabets[alphabet_name].as_str().unwrap();

            // Alphabet works as unicode
            let decoded = decode(alphabet, input).unwrap();
            let encoded = encode(alphabet, &decoded);
            assert_eq!(encoded, input);

            // Alphabet works as ASCII
            let decoded = decode(alphabet.as_bytes(), input).unwrap();
            let encoded = encode(alphabet.as_bytes(), &decoded);
            assert_eq!(encoded, input);
        }
    }

    #[test]
    fn is_unicode_sound() {
        // binary, kinda...
        let alphabet = "😐😀";

        let encoded = encode(alphabet, &[0xff, 0x00, 0xff, 0x00]);
        let decoded = decode(alphabet, &encoded).unwrap();

        assert_eq!(
            encoded,
            "😀😀😀😀😀😀😀😀😐😐😐😐😐😐😐😐😀😀😀😀😀😀😀😀😐😐😐😐😐😐😐😐"
        );
        assert_eq!(decoded, &[0xff, 0x00, 0xff, 0x00]);
    }

    #[test]
    #[should_panic(expected = "at least 2")]
    fn encode_empty_alphabet_panics() {
        encode("", &[1u8]);
    }

    #[test]
    #[should_panic(expected = "at least 2")]
    fn encode_empty_alphabet_bytes_panics() {
        encode(b"".as_slice(), &[1u8]);
    }

    #[test]
    #[should_panic(expected = "at least 2")]
    fn encode_single_char_alphabet_panics() {
        encode("x", &[1u8]);
    }

    #[test]
    #[should_panic(expected = "at least 2")]
    fn encode_single_char_alphabet_bytes_panics() {
        encode(b"x".as_slice(), &[1u8]);
    }

    #[test]
    #[should_panic(expected = "duplicate")]
    fn encode_duplicate_alphabet_panics() {
        encode("aab", &[3u8]);
    }

    #[test]
    #[should_panic(expected = "duplicate")]
    fn encode_duplicate_alphabet_bytes_panics() {
        encode(b"aab".as_slice(), &[3u8]);
    }

    #[test]
    fn decode_empty_alphabet_returns_error() {
        assert!(decode("", "abc").is_err());
    }

    #[test]
    #[should_panic(expected = "duplicate")]
    fn decode_duplicate_alphabet_panics() {
        let _ = decode("aab", "aa");
    }

    #[test]
    #[should_panic(expected = "duplicate")]
    fn decode_duplicate_alphabet_bytes_panics() {
        let _ = decode(b"aab".as_slice(), "aa");
    }
}
