//! Number Names
//!
//! For a given integer, this crate will return a string of either the cardinal ("one") or
//! ordinal ("first") String corresponding to that value.
//!
//! The algorithm used for the original implementation of this library was adapted from
//! [this post](https://stackoverflow.com/a/61604407/2313245).
//!
//! Example:
//! ```
//! use number_names::NumberName;
//!
//! assert_eq!(NumberName(10).cardinal(), "ten");
//! //assert_eq!(ordinal, "tenth");
//! ```


// BEGIN [this section contains code adapted from (https://stackoverflow.com/a/61604407/2313245)]
use std::iter::successors;

const ONES: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty",
    "seventy", "eighty", "ninety",
];
const ORDERS: [&str; 7] = [
    "zero", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion",
];

/// Wrapper struct for number name formatting
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NumberName(pub u64);

impl NumberName {
    /// Provides the cardinal name for the stored number
    pub fn cardinal(&self) -> String {
        self.encode(self.0)
    }

    fn encode(&self, num: u64) -> String {
        match num {
            0..=19 => ONES[num as usize].to_string(),
            20..=99 => {
                let upper = (num / 10) as usize;
                match num % 10 {
                    0 => TENS[upper].to_string(),
                    lower =>
                        format!("{}-{}", TENS[upper], self.encode(lower)),
                }
            }
            100..=999 => self.format_num(num, 100, "hundred"),
            _ => {
                let (div, order) =
                    successors(Some(1u64), |v| v.checked_mul(1000))
                        .zip(ORDERS.iter())
                        .find(|&(e, _)| e > num / 1000)
                        .unwrap();

                self.format_num(num, div, order)
            }
        }
    }

    fn format_num(&self, num: u64, div: u64, order: &str) -> String {
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
        let values = [
            (0,
             "zero",
             "zeroth"),
            (1,
             "one",
             "first"),
            (2,
             "two",
             "second"),
            (34,
             "thirty-four",
             "thirty-fourth"),
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
             "two hundred forty-two million give hundred twenty-six thousand two hundred seventy-second"),
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
        ];

        for value in values {
            assert_eq!(value.1.to_string(), NumberName(value.0).cardinal(), "Failed on {}", value.0);
        }
    }
}
