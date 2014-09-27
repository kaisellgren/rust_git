use commit::Commit;
use std::str::UnicodeStrSlice;
use std::from_str::FromStr;
use reader::Reader;

pub trait Serializable {
    fn encode(&self) -> Vec<u8>;
    fn encode_body(&self) -> Vec<u8>;
}

pub fn encode_author_info(commit: &Commit) -> Vec<u8> {
    (format!("author {} <{}> {}\n",
             commit.author_name,
             commit.author_email,
             encode_date(commit.author_date).as_slice()))
    .into_bytes()
}

pub fn encode_commit_info(commit: &Commit) -> Vec<u8> {
    (format!("committer {} <{}> {}\n",
             commit.committer_name,
             commit.committer_email,
             encode_date(commit.commit_date).as_slice()))
    .into_bytes()
}

pub fn decode_user_info(reader: &mut Reader) -> (String, String, uint) {
    let name = reader.take_string_while(|&c| c != 60).trim();

    reader.skip(1); // <

    let email = reader.take_string_while(|&c| c != 62).trim();

    reader.skip(2); // One ´>´ and one space.

    let timestamp: uint = FromStr::from_str(reader.take_string_while(|&c| c != 32)).unwrap();

    reader.skip(1); // One space.

    let time_zone_offset = reader.take_string_while(|&c| c != 10);

    reader.skip(1); // LF.

    (name.into_string(), email.into_string(), timestamp)
}

pub fn encode_date(date: uint) -> String {
    format!("{} +0000", date)
}