#![allow(dead_code, unused_imports)]

extern crate flate;

mod commit;
mod meta;
mod object_id;
mod object_header;
mod object_type;
mod eobject;
mod unit_tests;
mod serialization;
mod reader;
mod conversion;
mod error;
mod file_status;
mod repository;
mod branch;
mod tag_type;
mod tag;
mod reference;
mod pack_file;
mod pack_index;
mod blob;
mod object_database;
mod commit_sort_strategy;
mod commit_filter;
mod extensions;