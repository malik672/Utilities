use hex::encode;

/*                          INSPECTION                         */
pub fn is_bytes(arr: &Vec<u8>) -> bool {
    !arr.is_empty()
}

pub fn is_bytes_like<T: AsRef<Vec<u8>>>(data: T) -> bool {
    let ref_bytes = data.as_ref();
    !ref_bytes.is_empty();
}

pub fn is_hex_string(strs: &str) -> bool {
    strs.chars().all(|c| c.is_digit(16))
}


/*                       Hex String Conversion               */
pub fn array_to_hex_string(arr: &Vec<u8>)  -> result<String, Box<dyn std::error::Error>>{
    let hex_string = encode(byte_vec);
    Ok(hex_string)
}