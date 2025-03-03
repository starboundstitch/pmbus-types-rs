# pmbus-types-rs

A small crate that does unit conversions for the SLinear11 and ULinear16 PMBus formats.

## Usage Instructions

Add this crate to your `cargo.toml` and the `slinear11` and `ulinear16` will be available.

The `from()` functions generate the type from float values and the `to()` functions return a `u16` with the data type bits.

## License
 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

## Contribution

Contributions are welcome.

Please make sure that the tests all pass when making changes to the existing data types.
