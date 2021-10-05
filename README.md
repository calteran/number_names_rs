# Number Names

[![build](https://github.com/calteran/number_names_rs/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/calteran/number_names_rs/actions/workflows/build.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![codecov](https://codecov.io/gh/calteran/number_names_rs/branch/main/graph/badge.svg?token=WVBHQ5O0MX)](https://codecov.io/gh/calteran/number_names_rs)

Number names is a Rust library to provide formatted string names for cardinal and ordinal numbers.

*At this time, only American English is supported, but there are future plans for i18n.*

## Example usage:

 ```rust
use number_names::NumberName;

assert_eq!(NumberName(10).cardinal(), "ten");
assert_eq!(NumberName(10).ordinal(), "tenth");
 ```

## Contributing

As this is my first project in Rust, I'm sure there are significant improvements to be made in both the algorithms
and implementation.  I will gladly accept any constructive criticisms, suggestions or pull requests that make
this small project more efficient or accurate.

## Roadmap

- [ ] Cover all integer sizes (currently works up to u64)
- [ ] Include negative numbers
- [ ] Refactor to allow third-party language contributions

## Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

### [Unreleased]


### [0.1.0] 2021-09-24
#### Added
- positive cardinal formatting up to `u64::MAX()` in en_US
- positive ordinal formatting up to `u64::MAX()` in en_US
