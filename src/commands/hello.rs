use crate::resp::RespType;

pub fn hello(args: &[RespType]) -> RespType {
    // Currently the standard are to use RESP2 for redis
    // https://redis.io/commands/hello/
    if let Some(RespType::BulkString(protover)) = args.first() {
        if protover != "2" {
            return RespType::Error(
                "NOPROTO sorry, this protocol version is not supported.".into(),
            );
        }
    }

    RespType::Array(vec![
        RespType::BulkString("server".into()),
        RespType::BulkString("rust-eez".into()),
        RespType::BulkString("version".into()),
        // TODO: Get this version from Cargo or some sort
        RespType::BulkString("0.1.0".into()),
        RespType::BulkString("version-name".into()),
        RespType::BulkString("hanabi".into()),
        RespType::BulkString("proto".into()),
        RespType::BulkString("2".into()),
    ])
}
