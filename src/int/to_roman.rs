use std::{
    fmt,
    ops::{Deref, DerefMut},
};

#[derive(Debug, PartialEq, Eq)]
pub struct RomanNumeral(String);

impl fmt::Display for RomanNumeral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl PartialEq<str> for RomanNumeral {
    fn eq(&self, other: &str) -> bool {
        self.0.as_str() == other
    }
}

impl Deref for RomanNumeral {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RomanNumeral {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl RomanNumeral {
    fn new(num: u32) -> RomanNumeral {
        let mut given_number = num;
        let numerals: [(u32, &str); 13] = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut result = RomanNumeral("".to_owned());
        for &(value, symbol) in numerals.iter() {
            let count = given_number / value;
            if count > 0 {
                let multiplied_string = symbol.repeat(count as usize);
                result.push_str(&multiplied_string);
                given_number = given_number % value;
            }
        }
        result
    }
}

pub fn to_roman(num: u32) -> RomanNumeral {
    RomanNumeral::new(num)
}

#[test]
fn convert_500() {
    assert_eq!(&to_roman(500), "D");
}

#[test]
fn convert_501() {
    assert_eq!(&to_roman(501), "DI");
}

#[test]
fn convert_709() {
    assert_eq!(&to_roman(709), "DCCIX");
}

#[test]
fn three() {
    assert_eq!(&to_roman(3), "III")
}
