use commit::Commit;
use std::str::FromStr;
use object_header::ObjectHeader;
use object_header;
use reader::Reader;
use has_meta::HasMeta;

pub fn encode(object: &HasMeta, body: &[u8]) -> Vec<u8> {
    let mut buff = Vec::new();
    let mut header = object.get_meta().header;

    if header.length == 0 {
        header.length = body.len();
    }

    buff.push_all(header.encode().as_slice());
    buff.push_all(body);

    buff
}

pub fn decode(bytes: &[u8]) -> (&[u8], ObjectHeader) {
    let header = object_header::decode(bytes);
    let data = bytes.slice_from(bytes.len() - header.length);

    (data, header)
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