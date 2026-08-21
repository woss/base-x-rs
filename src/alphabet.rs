#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};
use crate::DecodeError;

use crate::decoder::*;
use crate::encoder;

pub trait Alphabet {
    fn encode(self, input: &[u8]) -> String;

    fn decode(self, input: &str) -> Result<Vec<u8>, DecodeError>;
}

/// Check that every byte value in `alphabet` is unique.
fn assert_unique_bytes(alphabet: &[u8]) {
    let mut seen = [false; 256];
    for &b in alphabet {
        assert!(!seen[b as usize], "Alphabet must not contain duplicate characters");
        seen[b as usize] = true;
    }
}

/// Check that every char in `alphabet` is unique.
fn assert_unique_chars(alphabet: &[char]) {
    // For small alphabets a simple O(n²) scan avoids allocation.
    for (i, a) in alphabet.iter().enumerate() {
        for b in alphabet[i + 1..].iter() {
            assert!(a != b, "Alphabet must not contain duplicate characters");
        }
    }
}

impl<'a> Alphabet for &[u8] {
    #[inline(always)]
    fn encode(self, input: &[u8]) -> String {
        if !self.is_ascii() {
            panic!("Alphabet must be ASCII");
        }
        assert_unique_bytes(self);

        let mut out = encoder::encode(self, input);
        out.reverse();
        unsafe { String::from_utf8_unchecked(out) }
    }

    #[inline(always)]
    fn decode(self, input: &str) -> Result<Vec<u8>, DecodeError> {
        if !self.is_ascii() {
            return Err(DecodeError);
        }
        assert_unique_bytes(self);
        U8Decoder::new(self).decode(input)
    }
}

impl<'a> Alphabet for &str {
    #[inline(always)]
    fn encode(self, input: &[u8]) -> String {
        if self.is_ascii() {
            assert_unique_bytes(self.as_bytes());
            let mut out = encoder::encode(self.as_bytes(), input);
            out.reverse();
            unsafe { String::from_utf8_unchecked(out) }
        } else {
            let alphabet: Vec<char> = self.chars().collect();
            assert_unique_chars(&alphabet);
            let out = encoder::encode(&alphabet, input);
            out.iter().rev().collect()
        }
    }

    #[inline(always)]
    fn decode(self, input: &str) -> Result<Vec<u8>, DecodeError> {
        if self.is_ascii() {
            assert_unique_bytes(self.as_bytes());
            U8Decoder::new(self.as_bytes()).decode(input)
        } else {
            let alphabet: Vec<char> = self.chars().collect();
            assert_unique_chars(&alphabet);
            CharDecoder(&alphabet).decode(input)
        }
    }
}
