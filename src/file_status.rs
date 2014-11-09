//! Every index file status.

pub enum FileStatus {
    NonExistent,
    Current,
    New,
    Staged,
    Deleted,
    Renamed,
    IndexTypeChange,
    Untracked,
    Modified,
    Missing,
    TypeChanged,
    Ignored,
}