use std::{collections::HashMap};
use serde_json::Value;

pub struct BoltDocument {
    document : HashMap<i64, BoltDataObject>
}

pub struct BoltDataObject {
    data : Value
}