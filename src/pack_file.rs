use std::io::fs::File;

static EXTENDED_BIT_FLAG: &'static u8 = &0u8;
static COMMIT_BIT_FLAG: &'static u8 = &1u8;
static TREE_BIT_FLAG: &'static u8 = &2u8;
static BLOB_BIT_FLAG: &'static u8 = &3u8;
static TAG_BIT_FLAG: &'static u8 = &4u8;
static RESERVED_BIT_FLAG: &'static u8 = &5u8;
static OFFSET_DELTA: &'static u8 = &6u8;

pub struct PackFile {
    pub file: File,
}