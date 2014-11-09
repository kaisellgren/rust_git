//! RustGit provides facilities for reading, manipulating and creating Git repositories.
//!
//! ### Finding a changeset
//! ```rust,no_run
//! fn get_message_by_commit_id(id: &ObjectId) -> Result<String, GitError> {
//!     let path = Path::new("/path/to/repo");
//!     let repo = Repository::open(&path);
//!     commit::find_one(&id, &repo).map(|commit| commit.message)
//! }
//! ```

#![doc(html_root_url="https://kaisellgren.github.io/doc")]
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