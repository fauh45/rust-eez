use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{resp::RespType, storage::StorageType};

pub fn hset(args: &[RespType], storage: Arc<RwLock<HashMap<String, StorageType>>>) -> RespType {
    let mut args_iter = args.iter();

    let key = if let Some(RespType::BulkString(key)) = args_iter.next() {
        key
    } else {
        return RespType::Error("ARGERR key are required for HSET".into());
    };

    match storage.write() {
        Ok(mut storage_locked) => {
            let existing = storage_locked.get(key);
            let mut insert_count = 0;

            let mut existing_hash = if let Some(StorageType::HashMap(existing_hash)) = existing {
                existing_hash.clone()
            } else {
                HashMap::new()
            };

            while let (
                Some(RespType::BulkString(field_key)),
                Some(RespType::BulkString(field_value)),
            ) = (args_iter.next(), args_iter.next())
            {
                existing_hash.insert(field_key.into(), StorageType::String(field_value.into()));

                insert_count += 1;
            }

            storage_locked.insert(key.into(), StorageType::HashMap(existing_hash));

            RespType::Integer(insert_count)
        }
        Err(err) => {
            println!("[SetOp HSET] Got poisoned error for write: {:#?}", err);

            RespType::Error("ERR system error while inserting data".into())
        }
    }
}

pub fn hget(args: &[RespType], storage: Arc<RwLock<HashMap<String, StorageType>>>) -> RespType {
    let mut args_iter = args.iter();

    let (key, field_key) =
        if let (Some(RespType::BulkString(key)), Some(RespType::BulkString(field_key))) =
            (args_iter.next(), args_iter.next())
        {
            (key, field_key)
        } else {
            return RespType::Error("ARGERR key and field is required for HGET".into());
        };

    match storage.read() {
        Ok(storage_locked) => {
            if let Some(StorageType::HashMap(existing_hash)) = storage_locked.get(key) {
                if let Some(StorageType::String(field_value)) = existing_hash.get(field_key) {
                    RespType::BulkString(field_value.into())
                } else {
                    RespType::Null
                }
            } else {
                RespType::Null
            }
        }
        Err(err) => {
            println!("[SetOp HGET] Got poisoned storage for read: {:#?}", err);

            RespType::Error("ERR system error while getting data".into())
        }
    }
}
