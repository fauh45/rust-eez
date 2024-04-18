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

    // NOTE: This will check if the remaining length of the args (excluding the key)
    if args_iter.len() == 0 {
        return RespType::Error("ARGERR at least one field are required for HSET".into());
    }

    match storage.write() {
        Ok(mut storage_locked) => {
            let existing = storage_locked.get(key);
            let mut insert_count = 0;

            let mut existing_hash = if let Some(StorageType::HashMap(existing_hash)) = existing {
                existing_hash.clone()
            } else if existing.is_some() {
                return RespType::Error(
                    "WRONGTYPE operation against a key holding non-hashmap".into(),
                );
            } else {
                HashMap::new()
            };

            while let (
                Some(RespType::BulkString(field_key)),
                Some(RespType::BulkString(field_value)),
            ) = (args_iter.next(), args_iter.next())
            {
                existing_hash.insert(field_key.into(), field_value.into());

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
            let existing = storage_locked.get(key);
            if let Some(StorageType::HashMap(existing_hash)) = existing {
                if let Some(field_value) = existing_hash.get(field_key) {
                    RespType::BulkString(field_value.into())
                } else if existing.is_some() {
                    return RespType::Error(
                        "WRONGTYPE operation against a key holding non-hashmap".into(),
                    );
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

pub fn hgetall(args: &[RespType], storage: Arc<RwLock<HashMap<String, StorageType>>>) -> RespType {
    let key = if let Some(RespType::BulkString(key)) = args.first() {
        key
    } else {
        return RespType::Error("ARGERR key are required for HGETALL".into());
    };

    let mut return_values = Vec::<RespType>::new();

    match storage.read() {
        Ok(storage_locked) => {
            let existing = storage_locked.get(key);
            if let Some(StorageType::HashMap(existing_hash)) = existing {
                for (key, val) in existing_hash.iter() {
                    return_values.push(RespType::BulkString(key.into()));
                    return_values.push(RespType::BulkString(val.into()));
                }
            } else if existing.is_some() {
                return RespType::Error(
                    "WRONGTYPE operation against a key holding non-hashmap".into(),
                );
            }

            RespType::Array(return_values)
        }
        Err(err) => {
            println!("[SetOp HGETALL] Got poisoned storage for read: {:#?}", err);

            RespType::Error("ERR system error while getting data".into())
        }
    }
}

pub fn hdel(args: &[RespType], storage: Arc<RwLock<HashMap<String, StorageType>>>) -> RespType {
    let mut args_iter = args.iter();

    let key = if let Some(RespType::BulkString(key)) = args_iter.next() {
        key
    } else {
        return RespType::Error("ARGERR key are required for HDEL".into());
    };

    // NOTE: This will check if the remaining length of the args (excluding the key)
    if args_iter.len() == 0 {
        return RespType::Error("ARGERR at least one field are required for HDEL".into());
    }

    match storage.write() {
        Ok(mut storage_locked) => {
            let mut del_count = 0;
            let existing = storage_locked.get(key);

            let mut existing_hash = if let Some(StorageType::HashMap(existing_hash)) = existing {
                existing_hash.clone()
            } else if existing.is_some() {
                return RespType::Error(
                    "WRONGTYPE operation against a key holding non-hashmap".into(),
                );
            } else {
                return RespType::Integer(0);
            };

            while let Some(RespType::BulkString(key_to_delete)) = args_iter.next() {
                if existing_hash.remove(key_to_delete).is_some() {
                    del_count += 1;
                }
            }

            if existing_hash.is_empty() {
                storage_locked.remove(key);
            } else {
                storage_locked.insert(key.into(), StorageType::HashMap(existing_hash));
            }

            RespType::Integer(del_count)
        }
        Err(err) => {
            println!("[SetOp HDEL] Got poisoned error for write: {:#?}", err);

            RespType::Error("ERR system error while inserting data".into())
        }
    }
}
