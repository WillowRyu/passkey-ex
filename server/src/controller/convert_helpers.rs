use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

pub fn insert_into_hashmap<T>(
    map: &mut HashMap<String, Value>,
    key: &str,
    value: T,
) -> Result<(), serde_json::Error>
where
    T: Serialize,
{
    map.insert(key.to_string(), serde_json::to_value(value)?);
    Ok(())
}
