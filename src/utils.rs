//
// utils
//

use std::{num::ParseIntError, string::FromUtf8Error};

pub fn byte_array_to_string(bytes: &[u8]) -> Result<String, FromUtf8Error> {
    for (i, b) in bytes.iter().enumerate() {
        if *b == 0u8 {
            return String::from_utf8(bytes[..i].to_vec());
        }
    }
    return String::from_utf8(bytes[..].to_vec());
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

////////////////////////////////// C# Version String
//////////////////////////////////

pub struct Version {
    pub s: String,
}

impl Version {
    pub fn new<T: ToString>(s: T) -> Self {
        Self {
            s: s.to_string()
        }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.s.eq(&other.s)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut self_split = self.s.split(".").map(|x| i32::from_str_radix(x, 10).unwrap());
        let mut other_split = other.s.split(".").map(|x| i32::from_str_radix(x, 10).unwrap());
        let (major,minor,build,rev) = (self_split.next()?, self_split.next()?, self_split.next()?, self_split.next()?);
        let (omajor,ominor,obuild,orev) = (other_split.next()?, other_split.next()?, other_split.next()?, other_split.next()?);

        let mut is_greater = false;

        is_greater = rev > orev;

        if build > obuild {
            is_greater = true;
        }

        if build < obuild {
            is_greater = false;
        }

        if minor > ominor {
            is_greater = true;
        }

        if minor < ominor {
            is_greater = false;
        }

        if major > omajor {
            is_greater = true;
        }
        
        if major < omajor {
            is_greater = false;
        }

        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }

        if is_greater {
            return Some(std::cmp::Ordering::Greater);
        }

        Some(std::cmp::Ordering::Less)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn version() {
        use crate::utils::Version;

        let a = Version::new("0.6.9.1");
        let b = Version::new("0.6.8.1");
        let c = Version::new("0.6.9.1");
        let d = Version::new("0.7.1.1");

        assert!(a > b);
        assert!(a != b);
        assert!(b < a);
        assert!(a == c);
        assert!(a < d);
        assert!(d >= a);
    }
}

