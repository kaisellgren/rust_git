//! Represents Git identifiers.

use conversion;

pub static RAW_SIZE: uint = 20;
pub static HEX_SIZE: uint = 40;

#[deriving(PartialEq, Show, Clone)]
pub struct ObjectId {
    pub hash: String,
    bytes: Vec<u8>
}

impl ObjectId {
    pub fn from_string(hash: &str) -> ObjectId {
        ObjectId {
            hash: hash.to_string(),
            bytes: conversion::hex_string_to_bytes(hash)
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> ObjectId {
        ObjectId {
            hash: conversion::bytes_to_hex_string(bytes),
            bytes: bytes.into_vec()
        }
    }
}
