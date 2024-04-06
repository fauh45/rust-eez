use std::{io::Read, net::TcpStream};

#[derive(Debug)]
pub enum RespType {
    String(String),
    Integer(i64),
    // TODO: Separate the error to error code, and message
    Error(String),
    BulkString(String),
    Array(Vec<RespType>),
}

impl RespType {
    pub fn deserialize(mut stream: TcpStream) -> Result<Self, Box<dyn std::error::Error>> {
        let mut byte_buf = [0; 1];
        // Try to read the first bytes from the stream
        stream.read(&mut byte_buf)?;

        // Convert the byte into a string
        // NOTE: though might be better if it was a char?
        match core::str::from_utf8(&byte_buf) {
            // Match the first string to check for its type
            Ok(type_ident) => match type_ident {
                "+" => RespType::deserialize_string(stream),
                "-" => RespType::deserialize_error(stream),
                ":" => RespType::deserialize_integer(stream),
                // TODO: Create bulk string parser
                "$" => Ok(RespType::BulkString("Bulk String".into())),
                // TODO: Add array handler,
                "*" => Ok(RespType::Array(Vec::new())),
                _ => unimplemented!(),
            },
            Err(_) => unimplemented!(),
        }
    }

    fn deserialize_integer(mut stream: TcpStream) -> Result<Self, Box<dyn std::error::Error>> {
        let mut byte = 0 as u8;

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
                    stream.peek(std::slice::from_mut(&mut byte))?;

                    break;
                }
                _ => str_integer.push(byte as char),
            }
        }

        let mut final_integer: i64 = str_integer.parse()?;
        if !sign {
            final_integer = -final_integer;
        }

        Ok(Self::Integer(final_integer))
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
        // NOTE: There's gonna be better way to store a byte of data
        let mut byte_buf = [0; 1];
        let mut final_string = Vec::new();

        while let Ok(_) = stream.read(&mut byte_buf) {
            final_string.push(byte_buf[0]);
        }

        // Truncate the last "\r\n"
        final_string.truncate(final_string.len() - 2);

        match String::from_utf8(final_string) {
            Ok(str) => Ok(str.to_string()),
            Err(err) => Err(Box::new(err)),
        }
    }
}
