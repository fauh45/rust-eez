use std::{io::Read, net::TcpStream};

#[derive(Debug)]
pub enum RespType {
    String(String),
    Integer(u64),
    // TODO: Separate the error to error code, and message
    Error(String),
    BulkString(String),
    Array(Vec<RespType>),
}

impl RespType {
    pub fn deserialize(mut stream: TcpStream) -> std::io::Result<Self> {
        let mut byte_buf = [0; 1];
        // Try to read the first bytes from the stream
        stream.read(&mut byte_buf)?;

        // Convert the byte into a string
        // NOTE: though might be better if it was a char?
        match core::str::from_utf8(&byte_buf) {
            // Match the first string to check for its type
            Ok(type_ident) => match type_ident {
                // TODO: Add simple string parsing
                "+" => Ok(RespType::String("Bruh".into())),
                // TODO: Use simple string parsing to get the error
                "-" => Ok(RespType::Error("Some error!".into())),
                // TODO: Parse integer from stream
                ":" => Ok(RespType::Integer(69)),
                // TODO: Create bulk string parser
                "$" => Ok(RespType::BulkString("Bulk String".into())),
                // TODO: Add array handler,
                "*" => Ok(RespType::Array(Vec::new())),
                _ => unimplemented!(),
            },
            Err(_) => unimplemented!(),
        }
    }
}
