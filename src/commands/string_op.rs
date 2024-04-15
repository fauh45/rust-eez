use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{resp::RespType, storage::StorageType};

/// SET Command
///
/// Handle SET command based on Redis syntax. Set a string key, with a string value.
///
/// Currently implemented syntax
/// `SET key value`
pub fn set(args: &[RespType], storage: Arc<RwLock<HashMap<String, StorageType>>>) -> RespType {
    let mut args_iter = args.iter();

    let key_args = if let Some(RespType::BulkString(key_args)) = args_iter.next() {
        key_args
    } else {
        return RespType::Error("ARGERR no key are given for SET command".into());
    };

    let value_args = if let Some(RespType::BulkString(val_args)) = args_iter.next() {
        val_args
    } else {
        return RespType::Error("ARGERR no value are given for SET command".into());
    };

    match storage.write() {
        Ok(mut storage_locked) => {
            storage_locked.insert(
                key_args.to_string(),
                StorageType::String(value_args.to_string()),
            );

            RespType::String("OK".into())
        }
        Err(err) => {
            println!(
                "[StringOp SET] Got poisoned error on locking storage: {:#?}",
                err
            );

            RespType::Error("ERR system error while inserting data".into())
        }
    }
}

/// GET Command
///
/// Handle GET command, this command will get the value stored in the storage, then return it.
///
/// Currently implemented syntax
/// `GET key value`
pub fn get(args: &[RespType], storage: Arc<RwLock<HashMap<String, StorageType>>>) -> RespType {
    let key_args = if let Some(RespType::BulkString(key_args)) = args.first() {
        key_args
    } else {
        return RespType::Error("ARGERR no key are given for GET command".into());
    };

    match storage.read() {
        Ok(storage_locked) => match storage_locked.get(key_args) {
            Some(StorageType::String(value)) => RespType::BulkString(value.to_string()),
            _ => RespType::Null,
        },
        Err(err) => {
            println!(
                "[StringOp GET] Got poisoned error on locking storage: {:#?}",
                err
            );

            RespType::Error("ERR system error while getting data".into())
        }
    }
}

/// DEL Command
///
/// Handle DEL command, that will delete key(s) from the storage, then return how many key(s) are removed.
///
/// Currently implemented syntax
/// `DEL key [key ...]`
pub fn del(args: &[RespType], storage: Arc<RwLock<HashMap<String, StorageType>>>) -> RespType {
    if args.len() < 1 {
        return RespType::Error("ARGERR no keys given for DEL command".into());
    }

    let mut deleted_count = 0;

    match storage.write() {
        Ok(mut storage_locked) => {
            for key in args {
                if let RespType::BulkString(key) = key {
                    if let Some(_) = storage_locked.remove(key) {
                        deleted_count += 1;
                    }
                }
            }

            RespType::Integer(deleted_count)
        }
        Err(err) => {
            println!(
                "[StringOp DEL] Got poisoned error on locking storage: {:#?}",
                err
            );

            RespType::Error("ERR system error while deleting data".into())
        }
    }
}
