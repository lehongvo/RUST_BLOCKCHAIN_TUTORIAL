/// The function `try_add` in Rust attempts to add two `u128` values and returns the sum as an
/// `Option<u128>` if the result does not overflow.
/// 
/// Arguments:
/// 
/// * `left`: `u128` - an unsigned 128-bit integer representing the left operand for addition.
/// * `right`: The `right` parameter in the `try_add` function is of type `u128`, which represents an
/// unsigned 128-bit integer.
/// 
/// Returns:
/// 
/// The function `try_add` returns an `Option<u128>`. It returns `Some(add_value)` if the sum of `left`
/// and `right` is greater than both `left` and `right`, otherwise it returns `None`.
pub fn try_add(left: u128, right: u128) -> Option<u128> {
    let add_value = left + right;
    if add_value > left && add_value > right {
        return Some(add_value);
    } else {
        return None;
    }
}

/// The function `try_sub` in Rust attempts to subtract two `u128` values and returns the result if it
/// is greater than the second value, otherwise it returns `None`.
/// 
/// Arguments:
/// 
/// * `left`: The `left` parameter represents the value from which you want to subtract another value.
/// * `right`: The `right` parameter is the value that will be subtracted from the `left` parameter in
/// the `try_sub` function.
/// 
/// Returns:
/// 
/// The function `try_sub` returns an `Option<u128>`. It returns `None` if the `left` value is less than
/// the `right` value. If the subtraction result is greater than the `right` value, it returns
/// `Some(sub_value)`, where `sub_value` is the result of the subtraction. Otherwise, it returns `None`.
pub fn try_sub(left: u128, right: u128) -> Option<u128> {
    if left < right {
        return None;
    }
    let sub_value = left - right;
    if sub_value > right {
        return Some(sub_value);
    } else {
        return None;
    }
}

/// The function `try_mul` in Rust attempts to multiply two `u128` numbers and returns the result as an
/// `Option<u128>`, ensuring no overflow occurs.
/// 
/// Arguments:
/// 
/// * `left`: The `left` parameter is a 128-bit unsigned integer.
/// * `right`: The `right` parameter is a 128-bit unsigned integer value.
/// 
/// Returns:
/// 
/// The `try_mul` function returns an `Option<u128>`. It returns `None` if the `left` parameter is equal
/// to 0 or if the multiplication of `left` and `right` would result in an overflow. Otherwise, it
/// returns `Some(mul_value)`, where `mul_value` is the result of the multiplication.
pub fn try_mul(left: u128, right: u128) -> Option<u128> {
    if left == 0 {
        return None;
    }
    let mul_value = left * right;
    if mul_value / left == right {
        return Some(mul_value);
    } else {
        return None;
    }
}

/// The function `try_div` in Rust attempts to divide two `u128` numbers and returns the result as an
/// `Option<u128>`, or `None` if the division is not exact.
/// 
/// Arguments:
/// 
/// * `left`: The `left` parameter represents the dividend in a division operation.
/// * `right`: The `right` parameter represents the divisor in the division operation.
/// 
/// Returns:
/// 
/// The function `try_div` returns an `Option<u128>`. It returns `None` if the `right` parameter is
/// equal to 0 or if the division does not result in a whole number quotient. It returns
/// `Some(mul_value)` if the division results in a whole number quotient.
pub fn try_div(left: u128, right: u128) -> Option<u128> {
    if right == 0 {
        return None;
    }
    let mul_value = left / right;
    if mul_value * right == left {
        return Some(mul_value); 
    } else {
        return None;
    }
}

/// This Rust function calculates the remainder of dividing one unsigned 128-bit integer by another,
/// returning an `Option` with the result or `None` if the divisor is zero.
/// 
/// Arguments:
/// 
/// * `left`: The `left` parameter is a 128-bit unsigned integer.
/// * `right`: The `right` parameter is the divisor used in the modulo operation.
/// 
/// Returns:
/// 
/// The function `try_mod` returns an `Option<u128>`, which can either be `Some(u128)` if the `right`
/// value is not zero, or `None` if the `right` value is zero.
pub fn try_mod(left: u128, right: u128) -> Option<u128> {
    if right == 0 {
        return None;
    }
    let mul_value = left % right;
    return Some(mul_value);
}