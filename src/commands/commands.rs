use crate::resp::RespType;

use super::{hello::hello, ping::ping};

pub fn handle_commands(command_arr: Vec<RespType>) -> RespType {
    if let Some(RespType::BulkString(command_name)) = command_arr.first() {
        let command_args = &command_arr[1..];

        match command_name.as_str() {
            "PING" => ping(command_args),
            "HELLO" => hello(command_args),
            _ => RespType::Error(format!("ERR unknown command '{}'", command_name)),
        }
    } else {
        return RespType::Error(
            "WRONGTYPE wrong type, expected command name as a Bulk strings".into(),
        );
    }
}
