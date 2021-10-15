#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![doc = include_str!("../README.md")]

pub mod format;
pub mod languages;

use crate::format::Format;
use crate::languages::Language::*;

/// Return cardinal number name for a given number with the default format
///
/// Example:
/// ```rust
/// assert_eq!(number_names::cardinal(10), "ten".to_string())
/// ```
pub fn cardinal(num: u64) -> String {
    cardinal_with_fmt(num, Format::default())
}

/// Return cardinal number name for a given number with a given format
///
/// Example:
/// ```rust
/// use number_names::{cardinal_with_fmt, Format};
///
/// assert_eq(cardinal_with_fmt(20, Format::default()), "twenty".to_string());
/// ```
pub fn cardinal_with_fmt(num: u64, fmt: Format) -> String {
    match fmt.language {
        EnUs => crate::languages::en_us::cardinal_with_format(num, fmt),
        _ => crate::languages::en_us::cardinal_with_format(num, fmt),
    }
}

/// Return ordinal number name for a given number with the default format
///
/// Example:
/// ```rust
/// assert_eq!(number_names::ordinal(10), "tenth".to_string())
/// ```
pub fn ordinal(num: u64) -> String { ordinal_with_fmt(num, Format::default()) }

/// Return ordinal number name for a given number with a given format
///
/// Example:
/// ```rust
/// use number_names::{ordinal_with_fmt, Format};
///
/// assert_eq(ordinal_with_fmt(20, Format::default()), "twentieth".to_string());
/// ```
pub fn ordinal_with_fmt(num: u64, fmt: Format) -> String {
    match fmt.language {
        EnUs => crate::languages::en_us::ordinal_with_format(num, fmt),
        _ => crate::languages::en_us::ordinal_with_format(num, fmt),
    }
}

mod tests;
