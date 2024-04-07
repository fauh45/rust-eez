use crate::resp::RespType;

pub fn ping(args: &[RespType]) -> RespType {
    if let Some(RespType::BulkString(message)) = args.first() {
        return RespType::BulkString(message.into());
    }

    RespType::String("PONG".into())
}
