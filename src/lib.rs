use std::{
    collections::HashMap,
    io::{Read, Write},
    sync::{Arc, RwLock},
};

use storage::StorageType;

use crate::{commands::commands::handle_commands, resp::RespType};

pub mod commands;
pub mod resp;
pub mod storage;

pub fn handle_command_stream<S: Read + Write>(
    stream: S,
    storage: Arc<RwLock<HashMap<String, StorageType>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (resp, mut stream) = RespType::deserialize(stream)?;

    println!("[Main Handler] Parsed TCP packet: `{:?}`", resp);

    let response = if let RespType::Array(commands) = resp {
        handle_commands(commands, storage)
    } else {
        RespType::Error("WRONGTYPE array was expected".into())
    };

    println!("[Main Handler] Responding with `{:#?}`", response);
    stream.write_all(&response.serialize())?;

    Ok(())
}
