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
       let raw_signature = format!("{}{}{}", r,s,v);
       Some(raw_signature); 
    }else{
        None;
    }
}

pub fn split_signature(signature: &Str) -> Option<String> {
   if is_byte_string(signature){
     let bytes = 
   }
}