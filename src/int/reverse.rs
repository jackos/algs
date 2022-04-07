use std::{error::Error, fmt};

use funty::Integral;

#[derive(Debug, PartialEq)]
pub struct IntReverseError {
    details: String,
}

impl IntReverseError {
    fn new(msg: &str) -> IntReverseError {
        IntReverseError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for IntReverseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for IntReverseError {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Reverse any integer type, if that would result
pub fn reverse<T>(x: T) -> Result<T, IntReverseError>
where
    T: Integral,
{
    let _0: T = T::ZERO;
    let _2 = T::ONE + T::ONE;
    let _8 = _2 * _2 * _2;
    let _10 = _2 + _8;

    let mut res: T = T::ZERO;
    let mut rem = x;

    while rem != T::ZERO {
        let tmp = res.checked_mul(_10).ok_or(IntReverseError::new("Oh no"))?;
        res = tmp
            .checked_add(rem.rem(_10))
            .ok_or(IntReverseError::new("Oh no"))?;

        Some(rem = rem / _10);
    }
    Ok(res)
}

#[test]
fn reverse_works() {
    assert_eq!(reverse(54021), Ok(12045));
}

#[test]
fn reverse_neg_works() {
    assert_eq!(reverse(-54021), Ok(-12045));
}

#[test]
fn zero_from_out_of_bounds_add() {
    assert_eq!(reverse(1563847412), Err(IntReverseError::new("Oh no")));
}

#[test]
fn zero_from_out_of_bounds_mult() {
    assert_eq!(reverse(1534236469), Err(IntReverseError::new("Oh no")));
}
