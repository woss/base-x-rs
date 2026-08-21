# base-x

This is a Rust fork of https://github.com/cryptocoinjs/base-x

**WARNING:** This module is **NOT RFC3548** compliant,  it cannot be used for base16 (hex), base32, or base64 encoding in a standards compliant manner. 

And this my very first Rust project: please review the source code!

## Installation

Add this to `Cargo.toml` file:

```toml
[dependencies]
base-x = "0.3.0"
```

## Usage

```rust
extern crate base_x;

fn main() {
    let decoded = base_x::decode("01", "11111111000000001111111100000000").unwrap();
    let encoded = base_x::encode("01", &decoded);
    assert_eq!(encoded, "11111111000000001111111100000000");
}
```

## Changelog

- 0.3.0 (2026-08-21)

  - Added validation for alphabets: they must contain at least two unique
    characters, and byte-slice alphabets must be ASCII.
  - Updated the crate to Rust 2024 and refreshed development dependencies.
  - Replaced the `json` development dependency with `serde_json`.

- 0.2.11 (2022-06-24)

  Reduced the scope of unsafe code when converting internal integers to bytes.

- 0.2.10 (2022-04-23)

  Updated development dependencies and fixed compiler warnings.

- 0.2.8 (2020-11-08)

  Fixed the `std` feature configuration used by `no_std` builds.

- 0.2.7 (2020-11-03)

  Added `no_std` support; the `std` feature remains enabled by default.

- 0.2.5 (2019-05-30)

  Refactored encoding and decoding, fixed Clippy warnings, and added proper
  ASCII validation for byte-slice alphabets.

- 0.2.4 (2019-01-01)

  Updated package metadata.

- 0.2.3 (2018-08-24)

  Released the optimized internal big-integer implementation and UTF-8
  alphabet support.

- 0.2.0

  Breaking change: alphabet has to be provided as an array of bytes instead of a string.

- 0.1.0

  initial version

## Contributors

- [Friedel Ziegelmayer](https://github.com/dignifiedquire)
- [Maciej Hirsz](https://github.com/maciejhirsz)
