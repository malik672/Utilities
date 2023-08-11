//! Various utilities for manipulating Ethereum related data.
use num_bigint::BigUint;

pub fn from<T>(&param: T) -> Option<BigUint>
where
    T: Into<BigUint>, // Constraint: T must be convertible into BigUint
{
    let big_int: BigUint = param.into(); // Convert param into a BigUint
    Some(big_int) // Convert param into a BigUint
}

//add two  big numbers together
pub fn add(&num1:BigUint, &num2:BigUint) -> result<BigUint> {
   result = num1 + num2;
}

pub fn sub(&num1:BigUint, &num2:BigUint) -> result<BigUint> {
    result = num1 - num2;
}

pub fn mul(&num1:BigUint, &num2:BigUint) -> result<BigUint> {
    result = num1 * num2;
}

pub fn div(&num1:BigUint, &num2:BigUint) -> result<BigUint> {
    result = num1 / num2;
}

pub fn mods(&num1:BigUint, &num2:BigUint) -> result<BigUint> {
    result = num1 * num2;
}

pub fn pow(&num1:BigUint, &num2:BigUint) -> result<BigUint> {
    result = num1 * num2;
}

