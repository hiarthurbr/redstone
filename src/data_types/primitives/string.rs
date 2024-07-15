use std::str::FromStr;

use thiserror::Error;

use crate::data_types::{DataResult, Errors, SerDe};

use super::var_int::VarInt;

static MAX_SIZE: u32 = 32767;

/// Modified UTF-8 string prefixed with its size in bytes as a [`VarInt`].
///
/// Maximum length of `n` characters, which varies by context;
/// up to `n × 3` bytes can be used to encode `n` characters and both of those limits are checked.
/// Maximum `n` value is 32767.
/// The + 3 is due to the max size of a valid length [`VarInt`].
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct String {
    pub data: std::string::String,
    max_size: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Error)]
#[allow(clippy::module_name_repetitions)]
pub enum StringError {
    #[error("Invalid UTF-8 string")]
    InvalidUTF8,
    #[error("Invalid string length")]
    InvalidLength,
    #[error("String is too long")]
    OutOfBoundsLength,
    #[error("String encoding is too long")]
    OutOfBoundsEncoding,
}

impl String {
    /// Creates a new String with a maximum length of `len`.
    ///
    /// If `len` is not provided, the default is the maximum length, 32767.
    ///
    /// ## Errors
    ///
    /// Returns [`StringError::OutOfBoundsLength`] if `len` is greater than 32767.
    pub fn new(len: Option<u32>) -> DataResult<Self> {
        let len = len.unwrap_or(MAX_SIZE);
        if len > MAX_SIZE {
            return Err(StringError::OutOfBoundsLength)?;
        }

        Ok(String {
            data: std::string::String::with_capacity(len as usize),
            max_size: len,
        })
    }

    /// Inserts a string into the String struct.
    ///
    /// This replaces the current string.
    ///
    /// ## Errors
    ///
    /// Returns [`StringError::OutOfBoundsLength`] if `string` is greater than 32767 characters,
    /// or [`StringError::OutOfBoundsEncoding`] if the UTF-8 encoding of `string` is greater than 32767 bytes.
    pub fn insert(&mut self, string: std::string::String) -> DataResult<&mut Self> {
        if string.len() > self.max_size as usize {
            return Err(StringError::OutOfBoundsLength)?;
        }

        if string.as_bytes().len() > (MAX_SIZE * 3) as usize {
            return Err(StringError::OutOfBoundsEncoding)?;
        }

        self.data = string;

        Ok(self)
    }

    /// Concatenates a string into the String struct.
    ///
    /// Unlike [`String::insert`], this does not replace the current string,
    /// but instead appends the string to the current string.
    ///
    /// ## Errors
    ///
    /// Returns [`StringError::OutOfBoundsLength`] if `string` is greater than 32767 characters,
    /// if the current string length plus the length of `string` is greater than 32767,
    /// or [`StringError::OutOfBoundsEncoding`] if the UTF-8 encoding of `string` is greater than 32767 bytes.
    pub fn concat(&mut self, string: std::string::String) -> DataResult<&mut Self> {
        if string.len() > self.max_size as usize {
            return Err(StringError::OutOfBoundsLength)?;
        }

        if string.as_bytes().len() > (MAX_SIZE * 3) as usize {
            return Err(StringError::OutOfBoundsEncoding)?;
        }

        self.data = string;

        Ok(self)
    }
}

impl<'a> SerDe<'a> for String {
    type Input = &'a [u8];
    type Serialized = DataResult<Vec<u8>>;
    type Deserialized = DataResult<Self>;

    /// Encodes a String into bytes.
    ///
    /// ## Errors
    ///
    /// Returns [`StringError::OutOfBoundsLength`] if the length of the string overflows an i32,
    /// or [`VarIntError::EncodeOverflow`] if there is a problem encoding the [`VarInt`].
    fn encode(&self) -> Self::Serialized {
        let len = self
            .data
            .len()
            .try_into()
            .map_err(|_| StringError::OutOfBoundsLength)?;
        let mut varint_bytes = VarInt::new(len).encode()?;
        let mut string_bytes = self.data.clone().into_bytes();
        varint_bytes.append(&mut string_bytes);
        Ok(varint_bytes)
    }

    /// Decodes a String from bytes.
    ///
    /// ## Errors
    ///
    /// Returns [`StringError::InvalidUTF8`] if the bytes are not valid UTF-8,
    /// [`StringError::InvalidLength`] if the length of the string is not equal to the length specified by the [`VarInt`]
    /// or [`VarIntError::EncodeOverflow`] if there is a problem encoding the [`VarInt`].
    fn decode(data: Self::Input) -> Self::Deserialized {
        let mut position = 0;
        let size = VarInt::decode(&data[position..])?;

        position += size.encode()?.len();

        let data = std::string::String::from_utf8(data[position..].to_vec())
            .map_err(|_| StringError::InvalidUTF8)?;

        #[allow(clippy::cast_sign_loss)]
        if size.value() as usize != data.len() {
            return Err(StringError::InvalidLength)?;
        }

        Ok(String {
            data,
            #[allow(clippy::cast_sign_loss)]
            max_size: size.value() as u32,
        })
    }
}

impl FromStr for String {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > MAX_SIZE as usize {
            return Err(StringError::OutOfBoundsLength)?;
        }
        #[allow(clippy::cast_possible_truncation)]
        let len = s.len() as u32;
        let mut string = String::new(Some(len))?;
        string.insert(s.to_string())?;
        Ok(string)
    }
}

impl From<String> for std::string::String {
    fn from(string: String) -> Self {
        string.data
    }
}

#[cfg(test)]
mod test {
    use crate::data_types::{Errors, SerDe};
    use std::str::FromStr;

    use super::{String, StringError};

    #[test]
    fn encode_decode_string_001() {
        let original_string = "Hello, 你好, नमस्ते!";
        let utf8_string = String::from_str(original_string).unwrap();

        let bytes = utf8_string.encode();

        if let Ok(decoded_string) = String::decode(&bytes.unwrap()) {
            assert_eq!(decoded_string.data, original_string);
        } else {
            panic!("Failed to decode the string");
        }
    }

    #[test]
    fn encode_decode_string_002() {
        let original_string = "Hello, こんにちは, नमस्ते";
        let utf8_string = String::from_str(original_string).unwrap();

        let bytes = utf8_string.encode();

        if let Ok(decoded_string) = String::decode(&bytes.unwrap()) {
            assert_eq!(decoded_string.data, original_string);
        } else {
            panic!("Failed to decode the string");
        }
    }

    #[test]
    fn encode_decode_string_003() {
        let original_string = "Hello, สวัสดี, 你好";
        let utf8_string = String::from_str(original_string).unwrap();

        let bytes = utf8_string.encode();

        if let Ok(decoded_string) = String::decode(&bytes.unwrap()) {
            assert_eq!(decoded_string.data, original_string);
        } else {
            panic!("Failed to decode the string");
        }
    }

    #[test]
    fn encode_decode_string_004() {
        let original_string = "This is a valid string 🚀";
        let utf8_string = String::from_str(original_string).unwrap();

        let bytes = utf8_string.encode();

        if let Ok(decoded_string) = String::decode(&bytes.unwrap()) {
            assert_eq!(decoded_string.data, original_string);
        } else {
            panic!("Failed to decode the string");
        }
    }

    #[test]
    fn invalid_utf8_bytes_001() {
        // Providing invalid UTF-8 bytes to check error handling
        let invalid_bytes = [vec![0x0B], b"Hello, \xF0\x28\x8C\xBC".into()].concat();
        let result = String::decode(&invalid_bytes);
        assert_eq!(result, Err(Errors::StringError(StringError::InvalidUTF8)));
    }

    #[test]
    fn invalid_utf8_bytes_002() {
        // Providing invalid UTF-8 bytes to check error handling
        let invalid_bytes = [vec![0x02], vec![0xC0, 0xAF]].concat();
        let result = String::decode(&invalid_bytes);
        assert_eq!(result, Err(Errors::StringError(StringError::InvalidUTF8)));
    }

    #[test]
    fn invalid_utf8_bytes_003() {
        // Providing invalid UTF-8 bytes to check error handling
        let invalid_bytes = [vec![0x04], vec![0xF5, 0x80, 0x82, 0x83]].concat();
        let result = String::decode(&invalid_bytes);
        assert_eq!(result, Err(Errors::StringError(StringError::InvalidUTF8)));
    }

    #[test]
    fn invalid_length_001() {
        // The real string length is 12, but here we say it's 228
        let invalid_bytes: Vec<u8> = [vec![0xE4], b"Running fast".into()].concat();
        let result = String::decode(&invalid_bytes);
        assert_eq!(result, Err(Errors::StringError(StringError::InvalidLength)));
    }

    #[test]
    fn invalid_length_002() {
        // The real string length is 55, but here we say it's 123
        let invalid_bytes = [
            vec![0x7B],
            b"Traversing the mountain ranges to seek hidden treasures".into(),
        ]
        .concat();
        let result = String::decode(&invalid_bytes);
        assert_eq!(result, Err(Errors::StringError(StringError::InvalidLength)));
    }

    #[test]
    fn invalid_length_003() {
        // The real string length is 55, but here we say it's 161
        let invalid_bytes = [
            vec![0xA1],
            b"The symphony of nature orchestrates a mesmerizing dance".into(),
        ]
        .concat();
        let result = String::decode(&invalid_bytes);
        assert_eq!(result, Err(Errors::StringError(StringError::InvalidLength)));
    }

    #[test]
    fn invalid_length_004() {
        // The real string length is 10, but here we say it's 63
        let invalid_bytes = [vec![0x3F], b"Pizza time".into()].concat();
        let result = String::decode(&invalid_bytes);
        assert_eq!(result, Err(Errors::StringError(StringError::InvalidLength)));
    }

    #[test]
    fn invalid_length_005() {
        // The real string length is 55, but here we say it's 146
        let invalid_bytes = [
            vec![0x92],
            b"Lost in the maze of thoughts, seeking clarity and truth".into(),
        ]
        .concat();
        let result = String::decode(&invalid_bytes);
        assert_eq!(result, Err(Errors::StringError(StringError::InvalidLength)));
    }

    #[test]
    fn invalid_length_006() {
        // The real string length is 11, but here we say it's 200
        let invalid_bytes = [vec![0xC8], b"Hello world".into()].concat();
        let result = String::decode(&invalid_bytes);
        assert_eq!(result, Err(Errors::StringError(StringError::InvalidLength)));
    }

    #[test]
    fn invalid_length_007() {
        // The real string length is 8, but here we say it's 90
        let invalid_bytes = [vec![0x5A], b"Blue sky".into()].concat();
        let result = String::decode(&invalid_bytes);
        assert_eq!(result, Err(Errors::StringError(StringError::InvalidLength)));
    }

    #[test]
    fn invalid_length_008() {
        // The real string length is 13, but here we say it's 175
        let invalid_bytes = [vec![0xAF], b"Coding is fun".into()].concat();
        let result = String::decode(&invalid_bytes);
        assert_eq!(result, Err(Errors::StringError(StringError::InvalidLength)));
    }

    #[test]
    fn invalid_length_009() {
        let invalid_bytes = vec![0xFF, 0xFE, 0xFD, 0xFC];
        let result = String::decode(&invalid_bytes);
        assert_eq!(result, Err(Errors::StringError(StringError::InvalidLength)));
    }

    #[test]
    fn empty_string() {
        let empty_string = "";
        let utf8_string = String::from_str(empty_string).unwrap();

        let bytes = utf8_string.encode();

        if let Ok(decoded_string) = String::decode(&bytes.unwrap()) {
            assert_eq!(decoded_string.data, empty_string);
        } else {
            panic!("Failed to decode the empty string");
        }
    }

    #[test]
    fn round_trip() {
        let original_string = "Testing round-trip encoding and decoding!";
        let utf8_string = String::from_str(original_string).unwrap();

        let bytes = utf8_string.encode().unwrap();

        if let Ok(decoded_string) = String::decode(&bytes) {
            assert_eq!(decoded_string.data, original_string);
            let re_encoded_bytes = decoded_string.encode().unwrap();
            assert_eq!(re_encoded_bytes, bytes);
        } else {
            panic!("Failed to decode the string");
        }
    }
}
