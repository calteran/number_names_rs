#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![doc = include_str!("../README.md")]

// BEGIN [this section contains code adapted from (https://stackoverflow.com/a/61604407/2313245)]
use std::cmp;
use std::iter::successors;
use num_integer::Integer;

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
pub struct NumberName<I: Integer + Copy + PartialOrd>(pub I);

impl<I: Integer + Copy + PartialOrd> NumberName<I> {
    /// Provides the cardinal name for the stored number
    ///
    /// Example:
    /// ```rust
    /// use number_names::NumberName;
    ///
    /// assert_eq!(NumberName(11).cardinal(), "eleven".to_string());
    /// ```
    pub fn cardinal(&self) -> String {
        self.encode(self.0)
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

    fn encode(&self, num: I) -> String {
        match num as usize {
            0..=19 => ONES[num].to_string(),
            20..=99 => {
                let upper = (num / 10);
                match num  % 10 {
                    0 => TENS[upper].to_string(),
                    lower => format!("{}-{}", TENS[upper], self.encode(lower as I)),
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

#[cfg(test)]
mod tests {
    use crate::NumberName;
    #[test]
    fn cardinal_name() {
        let values = values();

        for value in values {
            assert_eq!(
                value.1.to_string(),
                NumberName(value.0).cardinal(),
                "Failed on {}",
                value.0
            );
        }
    }

    #[test]
    fn ordinal_name() {
        let values = values();

        for value in values {
            assert_eq!(
                value.2.to_string(),
                NumberName(value.0).ordinal(),
                "Failed on {}",
                value.0
            );
        }
    }

    fn values() -> [(u64, &'static str, &'static str); 49] {
        [
            (0,
             "zero",
             "zeroth"),
            (1,
             "one",
             "first"),
            (2,
             "two",
             "second"),
            (3,
             "three",
             "third"),
            (4,
             "four",
             "fourth"),
            (5,
             "five",
             "fifth"),
            (6,
             "six",
             "sixth"),
            (7,
             "seven",
             "seventh"),
            (8,
             "eight",
             "eighth"),
            (9,
             "nine",
             "ninth"),
            (10,
             "ten",
             "tenth"),
            (11,
             "eleven",
             "eleventh"),
            (12,
             "twelve",
             "twelfth"),
            (13,
             "thirteen",
             "thirteenth"),
            (14,
             "fourteen",
             "fourteenth"),
            (15,
             "fifteen",
             "fifteenth"),
            (16,
             "sixteen",
             "sixteenth"),
            (17,
             "seventeen",
             "seventeenth"),
            (18,
             "eighteen",
             "eighteenth"),
            (19,
             "nineteen",
             "nineteenth"),
            (20,
             "twenty",
             "twentieth"),
            (30,
             "thirty",
             "thirtieth"),
            (34,
             "thirty-four",
             "thirty-fourth"),
            (40,
             "forty",
             "fortieth"),
            (50,
             "fifty",
             "fiftieth"),
            (60,
             "sixty",
             "sixtieth"),
            (70,
             "seventy",
             "seventieth"),
            (80,
             "eighty",
             "eightieth"),
            (90,
             "ninety",
             "ninetieth"),
            (100,
             "one hundred",
             "one hundredth"),
            (567,
             "five hundred sixty-seven",
             "five hundred sixty-seventh"),
            (8_910,
             "eight thousand nine hundred ten",
             "eight thousand nine hundred tenth"),
            (11_121,
             "eleven thousand one hundred twenty-one",
             "eleven thousand one hundred twenty-first"),
            (314_151,
             "three hundred fourteen thousand one hundred fifty-one",
             "three hundred fourteen thousand one hundred fifty-first"),
            (6_171_819,
             "six million one hundred seventy-one thousand eight hundred nineteen",
             "six million one hundred seventy-one thousand eight hundred nineteenth"),
            (20_212_223,
             "twenty million two hundred twelve thousand two hundred twenty-three",
             "twenty million two hundred twelve thousand two hundred twenty-third"),
            (242_526_272,
             "two hundred forty-two million five hundred twenty-six thousand two hundred seventy-two",
             "two hundred forty-two million five hundred twenty-six thousand two hundred seventy-second"),
            (8_293_031_323,
             "eight billion two hundred ninety-three million thirty-one thousand three hundred twenty-three",
             "eight billion two hundred ninety-three million thirty-one thousand three hundred twenty-third"),
            (33_435_363_738,
             "thirty-three billion four hundred thirty-five million three hundred sixty-three thousand seven hundred thirty-eight",
             "thirty-three billion four hundred thirty-five million three hundred sixty-three thousand seven hundred thirty-eighth"),
            (394_041_424_344,
             "three hundred ninety-four billion forty-one million four hundred twenty-four thousand three hundred forty-four",
             "three hundred ninety-four billion forty-one million four hundred twenty-four thousand three hundred forty-fourth"),
            (4_546_474_849_505,
             "four trillion five hundred forty-six billion four hundred seventy-four million eight hundred forty-nine thousand five hundred five",
             "four trillion five hundred forty-six billion four hundred seventy-four million eight hundred forty-nine thousand five hundred fifth"),
            (15_253_545_556_575,
             "fifteen trillion two hundred fifty-three billion five hundred forty-five million five hundred fifty-six thousand five hundred seventy-five",
             "fifteen trillion two hundred fifty-three billion five hundred forty-five million five hundred fifty-six thousand five hundred seventy-fifth"),
            (859_606_162_636_465,
             "eight hundred fifty-nine trillion six hundred six billion one hundred sixty-two million six hundred thirty-six thousand four hundred sixty-five",
             "eight hundred fifty-nine trillion six hundred six billion one hundred sixty-two million six hundred thirty-six thousand four hundred sixty-fifth"),
            (6_667_686_970_717_273,
             "six quadrillion six hundred sixty-seven trillion six hundred eighty-six billion nine hundred seventy million seven hundred seventeen thousand two hundred seventy-three",
             "six quadrillion six hundred sixty-seven trillion six hundred eighty-six billion nine hundred seventy million seven hundred seventeen thousand two hundred seventy-third"),
            (74_757_677_787_980_818,
             "seventy-four quadrillion seven hundred fifty-seven trillion six hundred seventy-seven billion seven hundred eighty-seven million nine hundred eighty thousand eight hundred eighteen",
             "seventy-four quadrillion seven hundred fifty-seven trillion six hundred seventy-seven billion seven hundred eighty-seven million nine hundred eighty thousand eight hundred eighteenth"),
            (283_848_586_878_889_909,
             "two hundred eighty-three quadrillion eight hundred forty-eight trillion five hundred eighty-six billion eight hundred seventy-eight million eight hundred eighty-nine thousand nine hundred nine",
             "two hundred eighty-three quadrillion eight hundred forty-eight trillion five hundred eighty-six billion eight hundred seventy-eight million eight hundred eighty-nine thousand nine hundred ninth"),
            (1_929_394_959_697_989_910,
             "one quintillion nine hundred twenty-nine quadrillion three hundred ninety-four trillion nine hundred fifty-nine billion six hundred ninety-seven million nine hundred eighty-nine thousand nine hundred ten",
             "one quintillion nine hundred twenty-nine quadrillion three hundred ninety-four trillion nine hundred fifty-nine billion six hundred ninety-seven million nine hundred eighty-nine thousand nine hundred tenth"),
            (18_446_744_073_709_551_615,
             "eighteen quintillion four hundred forty-six quadrillion seven hundred forty-four trillion seventy-three billion seven hundred nine million five hundred fifty-one thousand six hundred fifteen",
             "eighteen quintillion four hundred forty-six quadrillion seven hundred forty-four trillion seventy-three billion seven hundred nine million five hundred fifty-one thousand six hundred fifteenth"),
            (1_000_000_000,
             "one billion",
             "one billionth")
        ]
    }
}
