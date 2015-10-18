//! `rust_git` provides facilities for reading, manipulating and creating Git repositories.
//!
//! The project is still in its early stages.
//!
//! # Examples
//! ### Finding a changeset
//! The following example finds a changeset and returns its message.
//!
//! ```rust,no_run
//! use rust_git::object_id::ObjectId;
//! use rust_git::error::GitError;
//! use rust_git::repository::Repository;
//! use rust_git::commit;
//!
//! fn get_message_by_commit_id(id: &ObjectId) -> Result<String, GitError> {
//!     let path = Path::new("/path/to/repo");
//!     let repo = try!(Repository::open(&path));
//!     commit::find_one(id, &repo).map(|commit| commit.message)
//! }
//! ```

#![doc(html_root_url="https://kaisellgren.github.io/doc")]
#![allow(dead_code)]

extern crate flate;
extern crate serialize;

pub mod commit;
pub mod meta;
pub mod object_id;
pub mod object_header;
pub mod object_type;
mod git_object;
mod unit_tests;
mod serialization;
mod reader;
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
mod file_util;
mod has_meta;
mod reference_collection;
pub mod result;
pub mod config;