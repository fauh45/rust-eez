use std::{io::Read, net::TcpStream};

/// RESP2 Compatible Enum
///
/// This enum should be able to represent
#[derive(Debug)]
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
    pub fn deserialize(
        mut stream: TcpStream,
    ) -> Result<(Self, TcpStream), Box<dyn std::error::Error>> {
        let mut byte = 0u8;
        // Try to read the first bytes from the stream
        stream.read(std::slice::from_mut(&mut byte))?;

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

    fn deserialize_array(
        stream: TcpStream,
    ) -> Result<(Self, TcpStream), Box<dyn std::error::Error>> {
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

    fn deserialize_bulk_string(
        stream: TcpStream,
    ) -> Result<(Self, TcpStream), Box<dyn std::error::Error>> {
        let (size, mut stream) = RespType::deserialize_number(stream)?;
        println!("[RespType BulkString] Reading for size: {:#?}", size);

        if size < 0 {
            return Ok((RespType::Null, stream));
        }

        let mut final_string = String::with_capacity(size.try_into()?);

        for _ in 0..size {
            let mut byte = 0u8;
            stream.read(std::slice::from_mut(&mut byte))?;

            final_string.push(byte as char);
        }

        println!(
            "[RespType BulkString] Reading final string: {:#?}",
            final_string
        );

        // Read the remaining "\r\n"
        stream.read(&mut [0u8; 2])?;

        Ok((RespType::BulkString(final_string), stream))
    }

    fn deserialize_integer(
        stream: TcpStream,
    ) -> Result<(Self, TcpStream), Box<dyn std::error::Error>> {
        match RespType::deserialize_number(stream) {
            Ok((num, stream)) => Ok((RespType::Integer(num), stream)),
            Err(err) => Err(err),
        }
    }

    /// Deserialize number from RESP2 formatting
    ///
    /// This function expects the following format left in the stream,
    /// "[< + | - >]< value >\r\n"
    fn deserialize_number(
        mut stream: TcpStream,
    ) -> Result<(i64, TcpStream), Box<dyn std::error::Error>> {
        let mut byte = 0u8;

        // If "+" then true, "-" then false
        let mut sign = true;
        let mut str_integer = String::new();

        while let Ok(_) = stream.read(std::slice::from_mut(&mut byte)) {
            match byte as char {
                '+' => sign = true,
                '-' => sign = false,
                '\r' => {
                    // There SHOULD be a check if the next byte is an '\n', though as it is not a string
                    // It should be safe to assume that the next byte would be the '\n', integer cannot be
                    // in a new line, right?
                    // Read the '\n' to clear it off from the stream
                    stream.read(std::slice::from_mut(&mut byte))?;

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

    fn deserialize_string(
        stream: TcpStream,
    ) -> Result<(Self, TcpStream), Box<dyn std::error::Error>> {
        match RespType::deserialize_simple_string(stream) {
            Ok((str, stream)) => Ok((RespType::String(str), stream)),
            Err(err) => Err(err),
        }
    }

    fn deserialize_error(
        stream: TcpStream,
    ) -> Result<(Self, TcpStream), Box<dyn std::error::Error>> {
        match RespType::deserialize_simple_string(stream) {
            Ok((str, stream)) => Ok((RespType::Error(str), stream)),
            Err(err) => Err(err),
        }
    }

    /// This function expects that the simple string that the first byte have been discarded.
    ///
    /// e.g. The raw string is, "+OK\r\n" or "-ERR Message\r\n", but the stream have string and the "\r\n" left, for example "OK\r\n".
    fn deserialize_simple_string(
        mut stream: TcpStream,
    ) -> Result<(String, TcpStream), Box<dyn std::error::Error>> {
        let mut byte = 0u8;
        let mut final_string = String::new();

        while let Ok(_) = stream.read(std::slice::from_mut(&mut byte)) {
            final_string.push(byte as char);
        }

        // Truncate the last "\r\n"
        final_string.truncate(final_string.len() - 2);

        Ok((final_string, stream))
    }
}
