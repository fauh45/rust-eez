use crate::resp::RespType;

pub fn ping(args: &[RespType]) -> RespType {
    let mut args_iter = args.iter();

    if let Some(RespType::BulkString(message)) = args_iter.next() {
        return RespType::BulkString(message.into());
    }

    RespType::String("PONG".into())
}
