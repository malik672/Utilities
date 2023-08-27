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

//object to hex
pub fn obj_to_hex_string(obj: &serde_json::Value) -> String {
    let values = obj.as_object();
    match values {
        Some(values) => {
            let hex_strings:Vec<String> = values.map(|values| {
              if let serde_json::Value::Number(num) = values{
                format!("{:02x}", num.as_u64().unwrap())
              }else {
                String::new() // Handle other cases as needed
              }
            }).collect();
            format!("0x{}", hex_strings.join(""))
        },
        None => println!("Value:{} type incorrect, it should be an object", values),
    }
    
}

//Array to hex
pub fn arr_to_hex(arr: Number) -> String {
  let hex_strings:Vec<String> = arr.map(|values| {
    format!("{:02x}", num)
  }).collect();

  hex_strings.join("");
}
