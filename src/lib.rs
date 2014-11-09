#![allow(dead_code, unused_imports)]

extern crate flate;

pub mod commit;
pub mod meta;
pub mod object_id;
pub mod object_header;
pub mod object_type;
mod eobject;
mod unit_tests;
mod serialization;
mod reader;
mod conversion;
pub mod error;
pub mod file_status;
pub mod repository;
pub mod branch;
pub mod tag_type;
pub mod tag;
pub mod reference;
mod pack_file;
mod pack_index;
pub mod blob;
pub mod object_database;
pub mod commit_sort_strategy;
pub mod commit_filter;
mod extensions;
mod file_util;
mod has_meta;
mod reference_collection;