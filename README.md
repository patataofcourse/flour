# flour

[![Version 2.0](https://img.shields.io/badge/version-v2.0-red)](https://github.com/patataofcourse/flour/releases/v2.0.0)
[![Crates.io](https://img.shields.io/badge/crates.io-v2.0-brightgreen)](https://crates.io/crates/flour/)
[![Documentation](https://img.shields.io/badge/docs.rs-v2.0-brightgreen)](https://docs.rs/flour/)

A crate that serializes and deserializes BCCAD / BRCAD files to and from a JSON format, with a library crate to parse these files

`flour` uses JSON with added support for `// single-line comments` and `/* multi-line comments */`.

## Resources
A specification of the BCCAD and BRCAD formats (known revisions from their respective Rhythm Heaven games) can be found in [SPECIFICATION.md](./SPECIFICATION.md)

## Building

To build the `flour` binary, you'll need to enable the `clap` feature, like so:
```sh
cargo build --features=clap 
```