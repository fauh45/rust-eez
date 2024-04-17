use std::{i64, io::Read};

/// RESP2 Compatible Enum
///
/// This enum should be able to represent
#[derive(Debug, PartialEq)]
pub enum RespType {
    /// Simple string
    String(String),
    /// Simple Integer
    Integer(i64),
    /// Simple Error
    ///
    /// Quite similar with Simple String, though have it's own formatting of "-ERR_CODE Error Message"
    // TODO: Separate the error to error code, and message
    Error(String),
    BulkString(String),
    /// Technically there's no NULL in RESP2 Specification, though as noted by
    /// [Bulk String specification](https://redis.io/docs/reference/protocol-spec/#bulk-strings) in RESP2,
    /// Bulk String with negative size is considered to be a null Bulk String.
    Null,
    Array(Vec<RespType>),
}

impl RespType {
    pub fn deserialize<S: Read>(mut stream: S) -> Result<(Self, S), Box<dyn std::error::Error>> {
        let mut byte = 0u8;
        // Try to read the first bytes from the stream
        stream.read_exact(std::slice::from_mut(&mut byte))?;

        // Convert the byte into a string
        // NOTE: though might be better if it was a char?
        match byte as char {
            // Match the first string to check for its type
            '+' => RespType::deserialize_string(stream),
            '-' => RespType::deserialize_error(stream),
            ':' => RespType::deserialize_integer(stream),
            '$' => RespType::deserialize_bulk_string(stream),
            '*' => RespType::deserialize_array(stream),
            _ => {
                println!("[RespType] Getting un-supported type: `{:?}`", byte as char);
                unimplemented!()
            }
        }
    }

    fn deserialize_array<S: Read>(stream: S) -> Result<(Self, S), Box<dyn std::error::Error>> {
        let (size, stream) = RespType::deserialize_number(stream)?;
        let mut array_content = Vec::<RespType>::with_capacity(size.try_into()?);

        let mut stream = stream;
        for _ in 0..size {
            let (deserialized_content, new_stream) = RespType::deserialize(stream)?;
            println!(
                "[RespType] Serialize result in: {:#?}",
                deserialized_content
            );

            array_content.push(deserialized_content);

            stream = new_stream;
        }

        Ok((RespType::Array(array_content), stream))
    }

    fn deserialize_bulk_string<S: Read>(
        stream: S,
    ) -> Result<(Self, S), Box<dyn std::error::Error>> {
        let (size, mut stream) = RespType::deserialize_number(stream)?;
        println!("[RespType BulkString] Reading for size: {:#?}", size);

        if size < 0 {
            return Ok((RespType::Null, stream));
        }

        let mut final_string = String::with_capacity(size.try_into()?);

        for _ in 0..size {
            let mut byte = 0u8;
            stream.read_exact(std::slice::from_mut(&mut byte))?;

            final_string.push(byte as char);
        }

        println!(
            "[RespType BulkString] Reading final string: {:#?}",
            final_string
        );

        // Read the remaining "\r\n"
        stream.read_exact(&mut [0u8; 2])?;

        Ok((RespType::BulkString(final_string), stream))
    }

    fn deserialize_integer<S: Read>(stream: S) -> Result<(Self, S), Box<dyn std::error::Error>> {
        match RespType::deserialize_number(stream) {
            Ok((num, stream)) => Ok((RespType::Integer(num), stream)),
            Err(err) => Err(err),
        }
    }

    /// Deserialize number from RESP2 formatting
    ///
    /// This function expects the following format left in the stream,
    /// "[< + | - >]< value >\r\n"
    fn deserialize_number<S: Read>(mut stream: S) -> Result<(i64, S), Box<dyn std::error::Error>> {
        let mut byte = 0u8;

        // If "+" then true, "-" then false
        let mut sign = true;
        let mut str_integer = String::new();

        while stream.read_exact(std::slice::from_mut(&mut byte)).is_ok() {
            match byte as char {
                '+' => sign = true,
                '-' => sign = false,
                '\r' => {
                    // There SHOULD be a check if the next byte is an '\n', though as it is not a string
                    // It should be safe to assume that the next byte would be the '\n', integer cannot be
                    // in a new line, right?
                    // Read the '\n' to clear it off from the stream
                    stream.read_exact(std::slice::from_mut(&mut byte))?;

                    break;
                }
                _ => str_integer.push(byte as char),
            }
        }

        let mut final_integer: i64 = str_integer.parse()?;
        if !sign {
            final_integer = -final_integer;
        }

        Ok((final_integer, stream))
    }

    fn deserialize_string<S: Read>(stream: S) -> Result<(Self, S), Box<dyn std::error::Error>> {
        match RespType::deserialize_simple_string(stream) {
            Ok((str, stream)) => Ok((RespType::String(str), stream)),
            Err(err) => Err(err),
        }
    }

    fn deserialize_error<S: Read>(stream: S) -> Result<(Self, S), Box<dyn std::error::Error>> {
        match RespType::deserialize_simple_string(stream) {
            Ok((str, stream)) => Ok((RespType::Error(str), stream)),
            Err(err) => Err(err),
        }
    }

    /// This function expects that the simple string that the first byte have been discarded.
    ///
    /// e.g. The raw string is, "+OK\r\n" or "-ERR Message\r\n", but the stream have string and the "\r\n" left, for example "OK\r\n".
    fn deserialize_simple_string<S: Read>(
        mut stream: S,
    ) -> Result<(String, S), Box<dyn std::error::Error>> {
        let mut byte = 0u8;
        let mut final_string = String::new();

        while stream.read(std::slice::from_mut(&mut byte)).is_ok() {
            final_string.push(byte as char);
        }

        // Truncate the last "\r\n"
        final_string.truncate(final_string.len() - 2);

        Ok((final_string, stream))
    }

    pub fn serialize(self) -> Vec<u8> {
        match self {
            Self::String(str) => Self::serialize_simple_string('+', str),
            Self::Error(err) => Self::serialize_simple_string('-', err),
            Self::Integer(num) => Self::serialize_simple_integer(num),
            Self::BulkString(str) => Self::serialize_bulk_string(str),
            Self::Array(arr) => Self::serialize_array(arr),
            // Special case, all Null will be a bulk string with minus length
            Self::Null => "$-1\r\n".into(),
        }
    }

    fn serialize_simple_string(prefix: char, str: String) -> Vec<u8> {
        // Create an array of bytes, with size of the string plus the prefix ('+' or '-') and the CRLF ("\r\n")
        let mut bytes = Vec::<u8>::with_capacity(str.len() + 3);

        bytes.push(prefix as u8);
        bytes.append(&mut str.into_bytes());
        bytes.append(&mut "\r\n".into());

        bytes
    }

    fn serialize_simple_integer(num: i64) -> Vec<u8> {
        let str_num: String = num.to_string();

        Self::serialize_simple_string(':', str_num)
    }

    fn serialize_bulk_string(str: String) -> Vec<u8> {
        let str_len = str.len().to_string();

        // Array of bytes with length of the string representation of the length + the string length + 2 CRLF ("\r\n") + 1 prefix ('$')
        let mut bytes = Vec::<u8>::with_capacity(str.len() + str_len.len() + 5);

        // Prefix
        bytes.push(b'$');

        // String Length
        bytes.append(&mut str_len.into_bytes());
        bytes.append(&mut "\r\n".into());

        // String data
        bytes.append(&mut str.into_bytes());
        bytes.append(&mut "\r\n".into());

        bytes
    }

    fn serialize_array(arr: Vec<Self>) -> Vec<u8> {
        let str_len = arr.len().to_string();

        // Array of bytes with length of at least the prefix ('*') + number of elements + 1 CRLF
        // As we don't know the total length of the rest of the array itself
        let mut bytes = Vec::<u8>::with_capacity(str_len.len() + 3);

        // Prefix
        bytes.push(b'*');

        // Array Length
        bytes.append(&mut str_len.into_bytes());
        bytes.append(&mut "\r\n".into());

        // For each item, serialize and append it
        for item in arr {
            bytes.append(&mut Self::serialize(item));
        }

        bytes
    }
}
