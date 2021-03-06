//! Represents Git changesets.

use object_id::ObjectId;
use object_header::ObjectHeader;
use meta::Meta;
use serialization;
use reader::Reader;
use error::GitError;
use error::GitError::CorruptCommit;
use error::GitError::NotFound;
use repository::Repository;
use commit_filter::CommitFilter;
use object_database::find_object_by_id;
use git_object::GitObject::GitCommit;
use commit_sort_strategy::CommitSortStrategy::MostRecent;
use has_meta::HasMeta;

#[deriving(PartialEq, Show, Clone)]
pub struct Commit {
    pub meta: Meta,
    pub author_name: String,
    pub author_email: String,
    pub author_date: uint,
    pub committer_name: String,
    pub committer_email: String,
    pub commit_date: uint,
    pub message: String,
    pub tree_id: ObjectId,
    pub parent_ids: Vec<ObjectId>,
}

impl HasMeta for Commit {
    fn get_meta(&self) -> &Meta {
        &self.meta
    }
}

pub fn encode(commit: &Commit) -> Vec<u8> {
    serialization::encode(commit, encode_body(commit).as_slice())
}

pub fn encode_body(commit: &Commit) -> Vec<u8> {
    let mut buff = Vec::new();

    buff.push_all(format!("tree {}\n", commit.tree_id.hash).into_bytes().as_slice());

    for id in commit.parent_ids.iter() {
        buff.push_all(format!("parent {}\n", id.hash).into_bytes().as_slice());
    }

    buff.push_all(serialization::encode_author_info(commit).as_slice());
    buff.push_all(serialization::encode_commit_info(commit).as_slice());

    buff.push_all(format!("\n{}", commit.message).into_bytes().as_slice());

    buff
}

pub fn decode(bytes: &[u8]) -> Result<Commit, GitError> {
    let (data, header) = try!(serialization::decode(bytes));
    decode_body(data, &header)
}

pub fn decode_body(bytes: &[u8], header: &ObjectHeader) -> Result<Commit, GitError> {
    let mut reader = Reader::from_data(bytes);

    if reader.take_string(5).unwrap_or("") != "tree " {
        return Err(CorruptCommit("Expected 'tree '".into_cow()))
    }

    let tree_id = try!(
        reader.take_string_based_object_id().or(Err(CorruptCommit("Invalid tree id".into_cow())))
    );

    reader.skip(1);

    let mut parent_ids = Vec::new();

    while reader.take_string_while(|&c| c != 32).unwrap_or("") == "parent" {
        reader.skip(1);

        parent_ids.push(try!(
            reader.take_string_based_object_id().or(Err(CorruptCommit("Invalid parent".into_cow())))
        ));

        reader.skip(1);
    }

    reader.back(6);

    if reader.take_string(7).unwrap_or("") != "author " {
        return Err(CorruptCommit("Expected 'author '".into_cow()))
    }

    let (author_name, author_email, author_date) = try!(serialization::decode_user_info(&mut reader));

    if reader.take_string(10).unwrap_or("") != "committer " {
        return Err(CorruptCommit("Expected 'committer '".into_cow()))
    }

    let (committer_name, committer_email, commit_date) = try!(serialization::decode_user_info(&mut reader));

    reader.skip(1);

    let message = try!(
        reader.get_rest_as_string().or(Err(CorruptCommit("Invalid commit message".into_cow())))
    );

    Ok(Commit {
        meta: Meta {
            id: try!(ObjectId::from_string("b744d5cddb5095249299d95ee531cbd990741140")),
            header: *header,
        },
        author_name: author_name,
        author_email: author_email,
        author_date: author_date,
        committer_name: committer_name,
        committer_email: committer_email,
        commit_date: commit_date,
        message: message.into_string(),
        tree_id: tree_id,
        parent_ids: parent_ids,
    })
}

pub fn find_one(id: &ObjectId, repository: &Repository) -> Result<Commit, GitError> {
    match try!(find_object_by_id(repository, id)) {
        box GitCommit(c) => Ok(c),
        _ => Err(NotFound),
    }
}

pub fn find(repository: &Repository, filter: CommitFilter) -> Result<Vec<Commit>, GitError> {
    let mut buffer = Vec::new();

    let since_ids = match filter {
        CommitFilter { since: Some(since), .. } => since,
        _ => Vec::new(),
    };

    match filter.sort {
        MostRecent => {
            let result: Result<Vec<Commit>, GitError> = since_ids.iter().map(|id| find_one(id, repository)).collect();
            let mut commits: Vec<Commit> = try!(result);

            loop {
                if commits.len() == 0 || buffer.len() == filter.limit {
                    break;
                }

                let most_recent: Commit = commits.iter().max_by(|c| c.commit_date).unwrap().clone();

                buffer.push(most_recent.clone());

                let index = commits.iter().position(|c| *c == most_recent).unwrap();
                commits.remove(index);

                let parents = {
                    let commit_id_matches = |id: &ObjectId| !commits.iter().any(|c| c.meta.id == *id);

                    let parent_commits: Result<Vec<Commit>, GitError> = most_recent.parent_ids.into_iter()
                        .filter(commit_id_matches)
                        .map(|id| find_one(&id, repository))
                        .collect();

                    try!(parent_commits)
                };

                commits.push_all(parents.as_slice());
            }

            Ok(buffer)
        },
        _ => Ok(buffer),
    }
}