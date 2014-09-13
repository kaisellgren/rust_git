use commit::Commit;

pub trait Serializable {
    fn encode(&self) -> Vec<u8>;
    fn encode_body(&self) -> Vec<u8>;
}

pub fn encode_author_info(commit: &Commit) -> Vec<u8> {
    (format!("author {} <{}> {}\n",
             commit.author_name,
             commit.author_email,
             commit.author_date))
    .into_bytes()
}

pub fn encode_commit_info(commit: &Commit) -> Vec<u8> {
    (format!("committer {} <{}> {}\n",
             commit.committer_name,
             commit.committer_email,
             commit.commit_date))
    .into_bytes()
}
