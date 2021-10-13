//
// utils
//

use std::string::FromUtf8Error;

pub fn byte_array_to_string(bytes: &[u8]) -> Result<String, FromUtf8Error> {
    for (i, b) in bytes.iter().enumerate() {
        if *b == 0u8 {
            return String::from_utf8(bytes[..i].to_vec());
        }
    }
    return String::from_utf8(bytes[..].to_vec());
}
