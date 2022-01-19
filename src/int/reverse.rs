pub fn reverse(x: i32) -> i32 {
    let mut res: i32 = 0;
    let mut rem: i32 = x;

    while rem != 0 {
        match res.checked_mul(10) {
            None => return 0,
            Some(tmp) => match tmp.checked_add(rem % 10) {
                None => return 0,
                Some(i) => res = i,
            },
        }
        rem = rem / 10;
    }
    res
}

#[test]
fn reverse_works() {
    assert_eq!(reverse(54021), 12045);
}

#[test]
fn reverse_neg_works() {
    assert_eq!(reverse(-54021), -12045);
}

#[test]
fn zero_from_out_of_bounds_add() {
    assert_eq!(reverse(1563847412), 0);
}

#[test]
fn zero_from_out_of_bounds_mult() {
    assert_eq!(reverse(1534236469), 0);
}
