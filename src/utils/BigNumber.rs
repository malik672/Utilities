//! Various utilities for manipulating Ethereum related data.
use num_bigint::BigUint;

pub fn from<T>(&param: T) -> Option<BigUint>
where
    T: Into<BigUint>, // Constraint: T must be convertible into BigUint
{
    let big_int: BigUint = param.into(); // Convert param into a BigUint
    Some(big_int) // Convert param into a BigUint
}


                                                                             /* MATH OPERATIONS */
//add two  big numbers together
pub fn add(num1: BigUint, num2: BigUint) -> Option<BigUint> {
    Some( num1 + num2);
}

pub fn sub(num1: BigUint, num2: BigUint) -> Option<BigUint> {
    if num1 >= num2 {
        Some(num1 - num2)
    } else {
        None
    }
}

pub fn mul(num1: BigUint, num2: BigUint) -> Option<BigUint> {
    Some(num1 * num2);
}

pub fn div(num1: BigUint, num2: BigUint) -> Option<BigUint> {
    if !num2.is_zero() {
        Some(num1 / num2)
    } else {
        None
    }
}

pub fn mods(num1: BigUint, num2: BigUint) -> result<BigUint> {
    if !num2.is_zero() {
        Some(num1 % num2)
    } else {
        None
    }
}

pub fn pow(base: BigUint, exponent: BigUint) -> result<BigUint> {
    result = base.pow(exponent);
}

/*                                                         COMPARISON AND EQUIVALENCE                                                            */

pub fn eq(num1: BigUint, num2: BigUint) -> result<bool> {
    result = num1 == num2;
}

pub fn lt(num1: BigUint, num2: BigUint) -> result<bool> {
    result = num1 < num2;
}

pub fn lte(num1: BigUint, num2: BigUint) -> result<bool> {
    result = num1 <= num2;
}

pub fn gt(num1: BigUint, num2: BigUint) -> result<bool> {
    result = num1 > num2;
}

pub fn gte(num1: BigUint, num2: BigUint) -> result<bool> {
    result = num1 >= num2;
}

pub fn is_zero(num1: BigUint) -> result<bool> {
    result = num1.is_zero();
}


