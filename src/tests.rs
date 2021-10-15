#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, "zero")]
    #[case(1, "one")]
    #[case(2, "two")]
    #[case(3, "three")]
    #[case(4, "four")]
    #[case(5, "five")]
    #[case(6, "six")]
    #[case(7, "seven")]
    #[case(8, "eight")]
    #[case(9, "nine")]
    #[case(10, "ten")]
    #[case(11, "eleven")]
    #[case(12, "twelve")]
    #[case(13, "thirteen")]
    #[case(14, "fourteen")]
    #[case(15, "fifteen")]
    #[case(16, "sixteen")]
    #[case(17, "seventeen")]
    #[case(18, "eighteen")]
    #[case(19, "nineteen")]
    #[case(20, "twenty")]
    #[case(30, "thirty")]
    #[case(34, "thirty-four")]
    #[case(40, "forty")]
    #[case(50, "fifty")]
    #[case(60, "sixty")]
    #[case(70, "seventy")]
    #[case(80, "eighty")]
    #[case(90, "ninety")]
    #[case(100, "one hundred")]
    #[case(567, "five hundred sixty-seven")]
    #[case(8_910, "eight thousand nine hundred ten")]
    #[case(11_121, "eleven thousand one hundred twenty-one")]
    #[case(314_151, "three hundred fourteen thousand one hundred fifty-one")]
    #[case(
        6_171_819,
        "six million one hundred seventy-one thousand eight hundred nineteen"
    )]
    #[case(
        20_212_223,
        "twenty million two hundred twelve thousand two hundred twenty-three"
    )]
    #[case(
        242_526_272,
        "two hundred forty-two million five hundred twenty-six thousand two hundred seventy-two"
    )]
    #[case(8_293_031_323, "eight billion two hundred ninety-three million thirty-one thousand three hundred twenty-three")]
    #[case(33_435_363_738, "thirty-three billion four hundred thirty-five million three hundred sixty-three thousand seven hundred thirty-eight")]
    #[case(394_041_424_344, "three hundred ninety-four billion forty-one million four hundred twenty-four thousand three hundred forty-four")]
    #[case(4_546_474_849_505, "four trillion five hundred forty-six billion four hundred seventy-four million eight hundred forty-nine thousand five hundred five")]
    #[case(15_253_545_556_575, "fifteen trillion two hundred fifty-three billion five hundred forty-five million five hundred fifty-six thousand five hundred seventy-five")]
    #[case(859_606_162_636_465, "eight hundred fifty-nine trillion six hundred six billion one hundred sixty-two million six hundred thirty-six thousand four hundred sixty-five")]
    #[case(6_667_686_970_717_273, "six quadrillion six hundred sixty-seven trillion six hundred eighty-six billion nine hundred seventy million seven hundred seventeen thousand two hundred seventy-three")]
    #[case(74_757_677_787_980_818, "seventy-four quadrillion seven hundred fifty-seven trillion six hundred seventy-seven billion seven hundred eighty-seven million nine hundred eighty thousand eight hundred eighteen")]
    #[case(283_848_586_878_889_909, "two hundred eighty-three quadrillion eight hundred forty-eight trillion five hundred eighty-six billion eight hundred seventy-eight million eight hundred eighty-nine thousand nine hundred nine")]
    #[case(1_929_394_959_697_989_910, "one quintillion nine hundred twenty-nine quadrillion three hundred ninety-four trillion nine hundred fifty-nine billion six hundred ninety-seven million nine hundred eighty-nine thousand nine hundred ten")]
    #[case(18_446_744_073_709_551_615, "eighteen quintillion four hundred forty-six quadrillion seven hundred forty-four trillion seventy-three billion seven hundred nine million five hundred fifty-one thousand six hundred fifteen")]
    #[case(1_000_000_000, "one billion")]
    fn cardinal_name(#[case] input: u64, #[case] expected: String) {
        assert_eq!(expected, cardinal(input))
    }

    #[test]
    fn ordinal_name() {
        let values = values();

        for value in values {
            assert_eq!(
                value.2.to_string(),
                ordinal(value.0),
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
