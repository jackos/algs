pub fn atoi(s: String) -> i32 {
    let s = s.trim();
    let negative = s.starts_with('-');
    let positive = s.starts_with('+');

    let mut x = s.chars();

    if negative || positive {
        x.next();
    }

    for c in x {
        if 
    }


    if negative {
        let x: i32 = match number.into() {
            Ok(i) => i,
            Err(_) => return -2147483648,
        };
        return x * -1;
    }
    match number.try_into() {
        Ok(i) => i,
        Err(_) => return 2147483647,
    }
}

#[test]
fn simple_number() {
    assert_eq!(atoi("2555".to_string()), 2555);
}

#[test]
fn spaces() {
    assert_eq!(atoi(" 2555 ".to_string()), 2555);
}

#[test]
fn negative_number() {
    // assert_eq!(atoi("-2555".to_string()), -2555);
}

#[test]
fn positive_number() {
    assert_eq!(atoi("+2555".to_string()), 2555);
}

#[test]
fn ignore_non_digits() {
    assert_eq!(atoi("2555asdf".to_string()), 2555);
}

#[test]
fn leading_zeros() {
    assert_eq!(atoi("00021145".to_string()), 21145);
}

#[test]
fn clamp_negative() {
    assert_eq!(atoi("-2147483650".to_string()), -2147483648);
}

#[test]
fn clamp_positive() {
    assert_eq!(atoi("2147483648".to_string()), 2147483647);
}

#[test]
fn clamp_massive_positive_number() {
    assert_eq!(atoi("21474838897897645657894648".to_string()), 2147483647);
}

#[test]
fn clamp_massive_negative_number() {
    assert_eq!(atoi("-21474838897897645657894648".to_string()), -2147483648);
}

#[test]
fn zero() {
    assert_eq!(atoi("0".to_string()), 0);
}
