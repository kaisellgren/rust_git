use super::config::KeyValue;
use super::config::ConfigEntries;
use std::collections::hash_map::HashMap;

pub fn write(entries: ConfigEntries) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();

    for (category, items) in entries.iter() {
        buffer.push_all("[".as_bytes());
        buffer.push_all(category.as_bytes());
        buffer.push_all("]\n".as_bytes());

        for &KeyValue(ref key, ref value) in items.iter() {
            buffer.push_all("	".as_bytes());
            buffer.push_all(key.as_bytes());
            buffer.push_all(" = ".as_bytes());
            buffer.push_all(value.as_bytes());
            buffer.push_all("\n".as_bytes());
        }
    }

    buffer
}