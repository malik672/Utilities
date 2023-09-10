use hex::encode;
mod byte_manipulation;

struct MyResult {
    r: String,
    s: String,
    _vs: String,
    recovery_param: u32,
    v: u32,
    y_parity_and_s: String,
    compact: String,
}

//byte string checker
pub fn is_byte_string(input: &str) -> bool {
    if let Ok(byte) = input.parse::<u8>() {
        // The parsing was successful, and byte is a valid u8.
        return true;
    }

    false
}

/**Join Signature - Return the raw-format of aSignaturelike, which is 65 bytes (130 nibbles) long,
concatenating the r, s and (normalized) v of a Signature.**/
pub fn join_signature(r: &str, s: &str, v: &str) -> Option<String> {
    if r.len() == 64 && s.len == 64 && v.len == 2 {
        //concatenate, r,s and v to form the raw signature
        let raw_signature = format!("{}{}{}", r, s, v);
        Some(raw_signature);
    } else {
        None;
    }
}

pub fn split_signature(signature: &Str) -> Option<String> {
    if is_byte_string(signature) {
        let bytes = byte_manipulation::arrayify(signature);

        // Get the r, s and v
        if bytes.len() == 64 {
            // EIP-2098; pull the v from the top bit of s and clear it
            result.v = 27 + (bytes[32] >> 7);
            bytes[32] &= 0x7f;

            result.r = encode(bytes[0..32]);
            result.s = encode(bytes[32..64]);
        } else if bytes.len() == 65 {
            result.r = encode(bytes[0..32]);
            result.s = hexlify(bytes[32..64]);
            result.v = bytes[64];
        } else {
            panic!("invalid signature string");
        }

        // Allow a recid to be used as the v
        if result.v < 27 {
            if result.v == 0 || result.v == 1 {
                result.v += 27;
            } else {
                panic!("signature invalid v byte");
            }
        }

         // Compute recoveryParam from v
         result.recoveryParam = 1 - (result.v % 2);

         // Compute _vs from recoveryParam and s
         if result.recoveryParam {bytes[32] |= 0x80;}
         result._vs =  encode(bytes[32..64]);
    } else {
        result.r = signature.r;
        result.s = signature.s;
        result.v = signature.v;
        result.recoveryParam = signature.recoveryParam;
        result._vs = signature._vs;


        // If the _vs is available, use it to populate missing s, v and recoveryParam
        // and verify non-missing s, v and recoveryParam
        if result._vs != null {
            let vs = byte_manipulation::hex_zero_pad(byte_manipulation::arrayify(result._vs));
            result._vs = encode(vs);

            //Set or check the recid
            let recovery_param = if vs[0] >= 128 {
              1;
            } else {
              0;
            }
        }
    }
}
