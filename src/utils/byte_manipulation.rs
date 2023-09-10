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
pub fn array_to_hex_string(arr: &Vec<u8>) -> result<String, Box<dyn std::error::Error>> {
    let hex_string = encode(byte_vec);
    Ok(hex_string)
}

//object to hex
pub fn obj_to_hex_string(obj: &serde_json::Value) -> String {
    let values = obj.as_object();
    match values {
        Some(values) => {
            let hex_strings: Vec<String> = values
                .map(|values| {
                    if let serde_json::Value::Number(num) = values {
                        format!("{:02x}", num.as_u64().unwrap())
                    } else {
                        String::new() // Handle other cases as needed
                    }
                })
                .collect();
            format!("0x{}", hex_strings.join(""))
        }
        None => {
            format!("Value:{} type incorrect, it should be an object", values)
        }
    }
}

// string to uint 8 array
pub fn arrayify(value: &str) -> Option<Vec<u8>> {
    let value = if value.starts_with("0x") {
        &value[2..]
    } else {
        value
    };

    if value.len() % 2 != 0 {
        return None;
    }

    let bytes = hex::decode(value);

    match bytes {
        Ok(parsed_bytes) => Some(parsed_bytes),
        Err(_) => None,
    }
}

//Array to hex
pub fn arr_to_hex(arr: Number) -> String {
    let hex_strings: Vec<String> = arr.map(|values| format!("{:02x}", num)).collect();

    hex_strings.join("");
}

//str to array
pub fn str_to_array(sig: &str) -> Vec<usize> {
    let mut result = Vec::new();

    for c in input_string.chars() {
        let code_point = c as usize;
        result.push(code_point);
    }

    result
}

//Hexstring Manipulation

//Concatenation - Concatenates all the BytesLike in arrayOfBytesLike into a single DataHexString
pub fn hex_concat(array_of_bytes_like: Vec<string>) -> String {
    let concatenated_hex = array_of_bytes_like.concat();

    concatenated_hex;
}

//Length - Returns the length (in bytes) of aBytesLike.
pub fn hex_data_length(a_bytes_like: &str) -> usize {
    let length = a_bytes_like.len();
    length;
}

//slice - Returns a DataHexString representation of a slice of aBytesLike
pub fn hex_data_slice(a_bytes_like: &str, start: usize, end: usize) -> Option<String> {
    // Check if start and end indices are within bounds
    let len = a_bytes_like.len();
    if start <= end && end <= len {
        let slice = &a_bytes_like[start..end];
        Some(slice.to_string())
    } else {
        None // Return None if indices are out of bounds
    }
}



//zeros strip - Returns a HexString representation of aBytesLike with all leading zeros removed.
pub fn hex_strip_zeroes(a_bytes_like: &str) -> Option<String> {
    // Use trim_start_matches to remove leading '0' characters and 'x'
    let stripped = a_bytes_like.trim_start_matches(|c| c == '0' || c == 'x');

    // Check if the stripped string is empty
    if stripped.is_empty() {
        None
    } else {
        Some(stripped.to_string())
    }
}

//zeros pad - Returns a DataHexString representation of aBytesLike padded to length bytes.
pub fn hex_zero_pad(a_bytes_like: &str, lenghth: usize) -> Option<String> {
    let current_length = a_bytes_like.len();

    if current_length > length {
        return None;
    }

    let padding = "0".repeat(length - current_length);

    let padded_string = format!("{}{}", padding, a_bytes_like);

    Some(padded_string)
}
