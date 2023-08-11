//! Various utilities for manipulating Ethereum related data.
use regex::Regex;

///check if the is the string is a valid address
pub fn is_address(address: &str) -> bool {
    let checker = Regex::new(r"^(0x)?[0-9a-fA-F]{40}$").unwrap();
    checker.is_match(address)
}
