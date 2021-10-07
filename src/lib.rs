#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![doc = include_str!("../README.md")]

// BEGIN [this section contains code adapted from (https://stackoverflow.com/a/61604407/2313245)]
use std::cmp;
use std::iter::successors;
use num::{Integer, ToPrimitive};

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
    "zero",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
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

/// Wrapper struct for number name formatting
///
/// Example:
///  ```rust
/// use number_names::NumberName;
///
/// assert_eq!(NumberName(10).cardinal(), "ten".to_string());
/// assert_eq!(NumberName(10).ordinal(), "tenth".to_string());
///  ```

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NumberName<I: Integer + ToPrimitive>(pub I);

impl<I: Integer + ToPrimitive> NumberName<I> {
    /// Provides the cardinal name for the stored number
    ///
    /// Example:
    /// ```rust
    /// use number_names::NumberName;
    ///
    /// assert_eq!(NumberName(11).cardinal(), "eleven".to_string());
    /// ```
    pub fn cardinal(&self) -> String {
        self.encode(self.0.to_usize().expect("failed to convert to usize!"))
    }

    /// Provides the ordinal name for the stored number
    ///
    /// Example:
    /// ```rust
    /// use number_names::NumberName;
    ///
    /// assert_eq!(NumberName(11).ordinal(), "eleventh".to_string());
    /// ```
    pub fn ordinal(&self) -> String {
        // Lets start with the cardinal version and then replace the last word
        let cardinal = self.cardinal();

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

    fn encode(&self, num: usize) -> String {
        match num {
            0..=19 => ONES[num].to_string(),
            20..=99 => {
                let upper = num / 10;
                match num  % 10 {
                    0 => TENS[upper].to_string(),
                    lower => format!("{}-{}", TENS[upper], self.encode(lower)),
                }
            }
            100..=999 => self.format_num(num, 100, "hundred"),
            _ => {
                let (div, order) = successors(Some(1usize), |v| v.checked_mul(1000))
                    .zip(ORDERS.iter())
                    .find(|&(e, _)| e > num / 1000)
                    .unwrap();

                self.format_num(num, div, order)
            }
        }
    }

    fn format_num(&self, num: usize, div: usize, order: &str) -> String {
        match (num / div, num % div) {
            (upper, 0) => format!("{} {}", self.encode(upper), order),
            (upper, lower) => {
                format!("{} {} {}", self.encode(upper), order, self.encode(lower))
            }
        }
    }
}
// END [this section contains code adapted from (https://stackoverflow.com/a/61604407/2313245)]

mod tests;