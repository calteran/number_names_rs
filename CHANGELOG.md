# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] 2021-10-15
### Changed
 - [**BREAKING API CHANGE**]: Removed the `NumberNames` struct; calls for names are now into simple functions
 - Rebuilt the algorithm for generating cardinal names
 - Moved name generating algorithms into specific `en_us` module to make room for future language additions

### Added
 - `Language` enum to specify the language desired for the given number name
 - `Format` struct, currently only configures the language

## [0.1.2] 2021-10-05
### Fixed
 - Changed license identifier to be picked up correctly by crates.io

## [0.1.1] 2021-10-05
### Added
 - Codecov build script
 - Separate changelog (this file)
### Changed
 - String concatenation style for ordinal numbers

## [0.1.0] 2021-09-24
### Added
- positive cardinal formatting up to `u64::MAX()` in en_US
- positive ordinal formatting up to `u64::MAX()` in en_US
