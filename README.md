# Number Names

[![build](https://github.com/calteran/number_names_rs/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/calteran/number_names_rs/actions/workflows/build.yml)

Number names is a Rust library to provide formatted string names for cardinal and ordinal numbers.

Current languages supported:
 - English (American)

## Example usage:

 ```rust
use number_names::NumberName;

assert_eq!(NumberName(10).cardinal(), "ten");
 ```
