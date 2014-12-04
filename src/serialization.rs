use commit::Commit;
use std::str::FromStr;
use object_header::ObjectHeader;
use object_header;
use reader::Reader;
use has_meta::HasMeta;
use error::GitError;
use error::GitError::CorruptObject;

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

pub fn decode(bytes: &[u8]) -> Result<(&[u8], ObjectHeader), GitError> {
    let header = try!(object_header::decode(bytes));
    let data = bytes.slice_from(bytes.len() - header.length);

    Ok((data, header))
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

pub fn decode_user_info(reader: &mut Reader) -> Result<(String, String, uint), GitError> {
    let name = try!(
        reader.take_string_while(|&c| c != 60).or(Err(CorruptObject("Invalid name".into_cow())))
    ).trim();

    reader.skip(1); // <

    let email = try!(
        reader.take_string_while(|&c| c != 62).or(Err(CorruptObject("Invalid email".into_cow())))
    ).trim();

    reader.skip(2); // One ´>´ and one space.

    let timestamp: uint = try!(
        FromStr::from_str(reader.take_string_while(|&c| c != 32).unwrap_or(""))
            .ok_or(CorruptObject("Invalid timestamp".into_cow()))
    );

    reader.skip(1); // One space.

    let time_zone_offset = try!(
        reader.take_string_while(|&c| c != 10).or(Err(CorruptObject("Invalid timezone".into_cow())))
    );

    reader.skip(1); // LF.

    Ok((name.into_string(), email.into_string(), timestamp))
}

pub fn encode_date(date: uint) -> String {
    format!("{} +0000", date)
}