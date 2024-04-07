use crate::resp::RespType;
use phf::phf_map;

use super::{hello::hello, ping::ping};

static COMMANDS_LIST: phf::Map<&'static str, fn(&[RespType]) -> RespType> = phf_map! {
    "PING" => ping,
    "HELLO" => hello
};

pub fn handle_commands(command_arr: Vec<RespType>) -> RespType {
    if let Some(RespType::BulkString(command_name)) = command_arr.first() {
        let command_handler = COMMANDS_LIST.get(&command_name);

        return match command_handler {
            Some(handler) => handler(&command_arr[1..]),
            _ => RespType::Error(format!("ERR unknown command '{}'", command_name)),
        };
    } else {
        return RespType::Error(
            "WRONGTYPE wrong type, expected command name as a Bulk strings".into(),
        );
    }
}
