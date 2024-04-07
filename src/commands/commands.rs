use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{resp::RespType, storage::StorageType};

use super::{hello::hello, ping::ping, string_op};

pub fn handle_commands(
    command_arr: Vec<RespType>,
    storage: Arc<RwLock<HashMap<String, StorageType>>>,
) -> RespType {
    if let Some(RespType::BulkString(command_name)) = command_arr.first() {
        let command_args = &command_arr[1..];

        match command_name.as_str() {
            "PING" => ping(command_args),
            "HELLO" => hello(command_args),
            "SET" => string_op::set(command_args, storage),
            "GET" => string_op::get(command_args, storage),
            "DEL" => string_op::del(command_args, storage),
            _ => RespType::Error(format!("ERR unknown command '{}'", command_name)),
        }
    } else {
        return RespType::Error(
            "WRONGTYPE wrong type, expected command name as a Bulk strings".into(),
        );
    }
}
