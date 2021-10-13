#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![doc = include_str!("../README.md")]

mod format;
mod languages;

use crate::format::Format;
use crate::languages::Language::*;

pub fn cardinal(num: u64) -> String {
    cardinal_with_fmt(num, Format::default())
}

pub fn cardinal_with_fmt(num: u64, fmt: Format) -> String {
    match fmt.language {
        EnUs => crate::languages::en_us::cardinal_with_format(num, fmt),
        _ => crate::languages::en_us::cardinal_with_format(num, fmt),
    }
}

pub fn ordinal(num: u64) -> String {
    ordinal_with_fmt(num, Format::default())
}

pub fn ordinal_with_fmt(num: u64, _fmt: Format) -> String {
    num.to_string()
}

mod tests;
