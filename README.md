# Number Names

[![build](https://github.com/calteran/number_names_rs/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/calteran/number_names_rs/actions/workflows/build.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![codecov](https://codecov.io/gh/calteran/number_names_rs/branch/main/graph/badge.svg?token=WVBHQ5O0MX)](https://codecov.io/gh/calteran/number_names_rs)
[![crates.io](https://img.shields.io/crates/v/number-names)](https://img.shields.io/crates/v/number-names)

Number names is a Rust library to provide formatted string names for cardinal and ordinal numbers.

*At this time, only American English is supported, but there are future plans for i18n.*

## Example usage:

 ```rust
assert_eq!(number_names::cardinal(10), "ten");
assert_eq!(number_names::ordinal(10), "tenth");
 ```

## Contributing

As this is my first project in Rust, I'm sure there are significant improvements to be made in both the algorithms
and implementation.  I will gladly accept any constructive criticisms, suggestions or pull requests that make
this small project more efficient or accurate.

More specific needs include expanding the library to more languages.  To add a language, create a file in the
/languages folder with the [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) for the language,
in snake_case.  Implement the `cardinal_with_format` and `ordinal_with_format` public functions, returning `String`s
for each `u64` number.  Add a tests module (either separately or in the same file) with tests covering several generic
as well as specific edge-cases for the language.  Add an option to the `number_names::languages::Language` enum with
the IETF tag in CamelCase.  Send me a pull request with the completed changes once all your tests are passing!

