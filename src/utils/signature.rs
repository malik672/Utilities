use hex::encode;
use byte_manipulation;

struct MyResult {
    r: String,
    s: String,
    v: u32,
    recovery_param: u32,
    _vs: String,
    y_parity_and_s: String,
    compact: String,
}

// Byte string checker
pub fn is_byte_string(input: &str) -> bool {
    input.chars().all(|c| c.is_ascii_hexdigit())
}

/** Join Signature - Return the raw-format of a Signature-like, which is 65 bytes (130 nibbles) long,
concatenating the r, s, and (normalized) v of a Signature. **/
pub fn join_signature(r: &str, s: &str, v: &str) -> Option<String> {
    if r.len() == 64 && s.len() == 64 && v.len() == 2 && is_byte_string(r) && is_byte_string(s) {
        // Concatenate r, s, and v to form the raw signature
        let raw_signature = format!("{}{}{}", r, s, v);
        Some(raw_signature)
    } else {
        None
    }
}

pub fn split_signature(signature: &str) -> MyResult {
    let mut result = MyResult {
        r: String::new(),
        s: String::new(),
        v: 0,
        recovery_param: 0,
        _vs: String::new(),
        y_parity_and_s: String::new(),
        compact: String::new(),
    };

    if is_byte_string(signature) {
        let bytes = byte_manipulation::arrayify(signature).unwrap(); // Handle errors properly

        if bytes.len() == 64 {
            // EIP-2098; pull the v from the top bit of s and clear it
            result.v = 27 + (bytes[32] >> 7);
            bytes[32] &= 0x7f;

            result.r = encode(&bytes[0..32]);
            result.s = encode(&bytes[32..64]);
        } else if bytes.len() == 65 {
            result.r = encode(&bytes[0..32]);
            result.s = encode(&bytes[32..64]);
            result.v = bytes[64] as u32;
        } else {
            panic!("Invalid signature string");
        }

        // Allow a recid to be used as the v
        if result.v < 27 {
            if result.v == 0 || result.v == 1 {
                result.v += 27;
            } else {
                panic!("Signature invalid v byte");
            }
        }

        // Compute recoveryParam from v
        result.recovery_param = 1 - (result.v % 2);

        // Compute _vs from recoveryParam and s
        if result.recovery_param == 1 {
            bytes[32] |= 0x80;
        }
        result._vs = encode(&bytes[32..64]);
    } else {
        // Handle other cases here
        // ...
    }

    result.y_parity_and_s = result._vs.clone();
    result.compact = format!("{}{}", result.r, &result.y_parity_and_s[2..]);

    result
}
