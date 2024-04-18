use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum StorageType {
    String(String),
    HashMap(HashMap<String, String>),
}
