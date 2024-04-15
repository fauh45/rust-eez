use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{resp::RespType, storage::StorageType};

use super::{hello::hello, ping::ping, set_op, string_op};

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
            "HSET" => set_op::hset(command_args, storage),
            "HGET" => set_op::hget(command_args, storage),
            "HGETALL" => set_op::hgetall(command_args, storage),
            "HDEL" => set_op::hdel(command_args, storage),
            _ => RespType::Error(format!("ERR unknown command '{}'", command_name)),
        }
    } else {
        RespType::Error("WRONGTYPE wrong type, expected command name as a Bulk strings".into())
    }
}
