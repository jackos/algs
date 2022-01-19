use std::cmp::max;
use std::collections::HashMap;

/// Loops through the string putting the current iterator
/// against the character in a hashmap.
/// It counts the total characters by subtracting the iterator
/// from where the first character started.
/// If the character is duplicated when inserting into the hashmap,
/// the new start becomes where the first duplicated character occurred,
/// So anything after that is still valid as unique string
pub fn longest_substring(s: String) -> i32 {
    let mut map = HashMap::new();

    let mut i = 0;
    let mut start = -1;
    let mut result = 0;

    for c in s.chars() {
        if let Some(last) = map.insert(c, i) {
            let last = last;
            start = max(start, last);
        }
        result = max(result, i - start);
        i += 1;
    }
    result
}

#[test]
fn long_substring() {
    let s = "abcsdfbasdba".to_string();
    let ans = longest_substring(s);
    assert_eq!(ans, 6);
}

#[test]
fn repeating_chars() {
    let s = "abba".to_string();
    let ans = longest_substring(s);
    assert_eq!(ans, 2);
}

#[test]
fn one_char() {
    let s = "a".to_string();
    let ans = longest_substring(s);
    assert_eq!(ans, 1);
}

#[test]
fn no_chars() {
    let s = "".to_string();
    let ans = longest_substring(s);
    assert_eq!(ans, 0);
}
