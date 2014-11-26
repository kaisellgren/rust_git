use std::io::File;
use result::EmptyResult;

pub fn create_file_with_contents(path: &Path, contents: &[u8]) -> EmptyResult {
    let mut file = File::create(path);
    Ok(try!(file.write(contents)))
}