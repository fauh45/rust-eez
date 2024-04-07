use std::{io::Read, net::TcpStream};

#[derive(Debug)]
pub enum RespType {
    String(String),
    Integer(i64),
    // TODO: Separate the error to error code, and message
    Error(String),
    BulkString(String),
    Null,
    Array(Vec<RespType>),
}

impl RespType {
    pub fn deserialize(mut stream: TcpStream) -> Result<Self, Box<dyn std::error::Error>> {
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
            // TODO: Add array handler,
            '*' => Ok(RespType::Array(Vec::new())),
            _ => unimplemented!(),
        }
    }

    fn deserialize_bulk_string(mut stream: TcpStream) -> Result<Self, Box<dyn std::error::Error>> {
        // NOTE: Also might be expensive to just clone it, might need to find a better handle the borrowing
        let size = RespType::deserialize_number(stream.try_clone()?)?;

        if size < 1 {
            return Ok(RespType::Null);
        }

        // NOTE: The size given might not be enough? Not really sure whats the limit of `usize` is.
        let mut final_string = String::with_capacity(size.try_into()?);
        stream.read_to_string(&mut final_string)?;

        // Read the remaining "\r\n"
        stream.read(&mut [0u8; 2])?;

        Ok(RespType::BulkString(final_string))
    }

    fn deserialize_integer(stream: TcpStream) -> Result<Self, Box<dyn std::error::Error>> {
        match RespType::deserialize_number(stream) {
            Ok(num) => Ok(RespType::Integer(num)),
            Err(err) => Err(err),
        }
    }

    /// Deserialize number from RESP2 formating
    ///
    /// This function expects the following format left in the stream,
    /// "[< + | - >]< value >\r\n"
    fn deserialize_number(mut stream: TcpStream) -> Result<i64, Box<dyn std::error::Error>> {
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

        Ok(final_integer)
    }

    fn deserialize_string(stream: TcpStream) -> Result<Self, Box<dyn std::error::Error>> {
        match RespType::deserialize_simple_string(stream) {
            Ok(str) => Ok(RespType::String(str)),
            Err(err) => Err(err),
        }
    }

    fn deserialize_error(stream: TcpStream) -> Result<Self, Box<dyn std::error::Error>> {
        match RespType::deserialize_simple_string(stream) {
            Ok(str) => Ok(RespType::Error(str)),
            Err(err) => Err(err),
        }
    }

    /// This function expects that the simple string that the first byte have been discarded.
    ///
    /// e.g. The raw string is, "+OK\r\n" or "-ERR Message\r\n", but the stream have string and the "\r\n" left, for example "OK\r\n".
    fn deserialize_simple_string(
        mut stream: TcpStream,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut byte = 0u8;
        let mut final_string = String::new();

        while let Ok(_) = stream.read(std::slice::from_mut(&mut byte)) {
            final_string.push(byte as char);
        }

        // Truncate the last "\r\n"
        final_string.truncate(final_string.len() - 2);

        Ok(final_string)
    }
}
