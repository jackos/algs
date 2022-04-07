#[inline]
pub fn a1(s: String) -> i32 {
    if s.is_empty() || s.len() == 0 {
        return 0;
    }

    let mut storage: String = "".to_string(); // Used to store the digits while iterating through the string.
    let mut is_negative = false;
    let mut special_char_visited = false;

    for c in s.chars() {
        if c.is_digit(10) == false {
            if storage.len() == 0 {
                // while no digit has been processed, there are certain special chars that can be
                // interpreted. (-,+, )
                if special_char_visited {
                    return 0;
                } // Only allowed 1 special char. Easy place to terminate.
                if c == ' ' {
                    continue;
                }
                if c == '-' {
                    is_negative = true;
                    special_char_visited = true;
                    continue;
                }
                if c == '+' {
                    is_negative = false;
                    special_char_visited = true;
                    continue;
                }
                return 0;
            } else {
                break;
            } // Stop processing the second you hit a non valid digit.
        }
        special_char_visited = true;
        if !(storage.len() == 0 && c == '0') {
            storage.push(c);
        }
    }

    if storage.len() == 0 {
        return 0;
    } // No chars in the string were valid digits.
    if storage.len() > 10 {
        // any number over 10 digits is going to be larger than a 32 bit int, no need to waste time
        // processing.
        return if is_negative {
            std::i32::MIN
        } else {
            std::i32::MAX
        };
    }

    let mut a: i64;
    match storage.parse::<i64>() {
        Ok(n) => a = n,
        Err(_n) => return 0,
    } // If all else fails, terminate method with 0.

    if is_negative {
        a = a * -1;
    }
    if a > i32::MAX as i64 {
        return std::i32::MAX;
    }
    if a < std::i32::MIN as i64 {
        return std::i32::MIN;
    }

    a as i32
}

#[inline]
pub fn a2(s: String) -> i32 {
    let ss = s.as_bytes();
    let len = ss.len();
    let mut i = 0;
    let mut sign = b'+';
    let mut val = 0i32;

    // Ignore leading whitespace.
    while i < len && ss[i] == b' ' {
        i += 1;
    }

    // Check for + / -.
    if i < len && (ss[i] == b'+' || ss[i] == b'-') {
        sign = ss[i];
        i += 1;
    }

    // Read to next non-digit or end, converting to negative numeric val.
    while i < len && ss[i] <= b'9' && ss[i] >= b'0' {
        val = val.saturating_mul(10).saturating_sub((ss[i] - b'0') as i32);
        i += 1;
    }
    // Adjust per sign.
    if sign == b'+' {
        val = val.saturating_neg();
    }
    val
}
