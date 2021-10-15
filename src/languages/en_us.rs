//! Generate number names in American English
use crate::format::Format;
use std::cmp;

/// Return cardinal number name for a given number with a given format in American English
///
/// Example:
/// ```rust
/// use number_names::{languages::en_us::cardinal_with_fmt, Format};
///
/// assert_eq(cardinal_with_fmt(20, Format::default()), "twenty".to_string());
/// ```
pub fn cardinal_with_format(num: u64, fmt: Format) -> String {
    encode(num, fmt)
}

/// Return cardinal number name for a given number with a given format in American English
///
/// Example:
/// ```rust
/// use number_names::{languages::en_us::ordinal_with_fmt, Format};
///
/// assert_eq(ordinal_with_fmt(20, Format::default()), "twentieth".to_string());
/// ```
pub fn ordinal_with_format(num: u64, fmt: Format) -> String {
    // Lets start with the cardinal version and then replace the last word
    let cardinal = cardinal_with_format(num, fmt);

    // find the character (this is all in ASCII!) before the last word
    // it might be a space or a dash (eg. "thirty-three")
    let last_break = match (cardinal.rfind(' '), cardinal.rfind('-')) {
        (Some(space), Some(dash)) => cmp::max(space, dash) + 1,
        (Some(space), None) => space + 1,
        (None, Some(dash)) => dash + 1,
        (None, None) => 0,
    };

    // Most numbers just add "-th" to the end of the cardinal name, but others are special
    match NOT_TH
        .iter()
        .position(|(word, _)| word.eq(&&cardinal[last_break..]))
    {
        Some(index) => String::from(&cardinal[..last_break]) + NOT_TH[index].1,
        None => cardinal + "th",
    }
}

fn encode(num: u64, fmt: Format) -> String {
    let mut num = num;
    if num > 0 {
        std::iter::from_fn(|| {
            if num > 0 {
                let v = num % 1000;
                num /= 1000;
                Some(v)
            } else {
                None
            }
        })
            .map(|num| format_num(num, &fmt))
            .zip(ORDERS.iter())
            .filter(|(num, _)| num != "zero")
            .map(|(num, order)| num + &order)
            .collect::<Vec<String>>()
            .into_iter().rev().collect::<Vec<String>>()
            .join(" ")
    }
    else {
        "zero".to_string()
    }
}

fn format_num (num: u64, fmt: &Format) -> String {
    match num {
        0..=19 => ONES[num as usize].to_string(),
        20..=99 => {
            let upper = (num / 10) as usize;
            match num % 10 {
                0 => TENS[upper].to_string(),
                lower => format!("{}-{}", TENS[upper], ONES[lower as usize]),
            }
        },
        _ => {
            let mut v =
                vec![format_num(num / 100, fmt), "hundred".to_string(), format_num(num % 100, fmt)];
            v.retain(|s| s != "zero");
            v.join(" ")
        }
    }
}

const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const ORDERS: [&str; 7] = [
    "",
    " thousand",
    " million",
    " billion",
    " trillion",
    " quadrillion",
    " quintillion",
];

const NOT_TH: [(&str, &str); 15] = [
    ("one", "first"),
    ("two", "second"),
    ("three", "third"),
    ("five", "fifth"),
    ("eight", "eighth"),
    ("nine", "ninth"),
    ("twelve", "twelfth"),
    ("twenty", "twentieth"),
    ("thirty", "thirtieth"),
    ("forty", "fortieth"),
    ("fifty", "fiftieth"),
    ("sixty", "sixtieth"),
    ("seventy", "seventieth"),
    ("eighty", "eightieth"),
    ("ninety", "ninetieth"),
];