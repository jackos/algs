use funty::Integral;

#[inline]
pub fn palindrome<T: Integral>(mut x: T) -> bool {
    if x == T::ZERO {
        return true;
    }
    // If it's a negative number, then convert it to a positive, MIN will return false
    // from the check as it's not a palindrome
    if x < T::ZERO {
        x = match x.checked_neg() {
            Some(i) => i,
            None => return false,
        }
    }
    let x = x.as_u128();
    let digit_count = x.log10() + 1;
    let half = (digit_count / 2) as usize;
    let digits = (0..digit_count).map(|exp| x / 10_u128.pow(exp) % 10);
    digits
        .clone()
        .take(half)
        .zip(digits.rev().take(half))
        .all(|(lhs, rhs)| lhs == rhs)
}

#[test]
fn is_palindrome_even_length() {
    assert_eq!(palindrome(632236), true)
}
#[test]
fn is_palindrome_odd_length() {
    assert_eq!(palindrome(6321236), true)
}

#[test]
fn is_not_palindrome() {
    assert_eq!(palindrome(122), false)
}

#[test]
fn is_palindrome_big() {
    assert_eq!(
        palindrome(222211111111111111111111111111111112222u128),
        true
    )
}

#[test]
fn is_not_palindrome_big() {
    assert_eq!(palindrome(u128::MAX), false)
}

#[test]
fn negative_false() {
    assert_eq!(palindrome(-10), false)
}

#[test]
fn negative_true() {
    assert_eq!(palindrome(-11011), true)
}

#[test]
fn negative_min_false() {
    assert_eq!(palindrome(i128::MIN), false)
}

#[test]
fn zero_true() {
    assert_eq!(palindrome(0), true)
}
