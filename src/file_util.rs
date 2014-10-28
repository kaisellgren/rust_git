use std::io::File;

pub fn create_file_with_contents(path: &Path, contents: &[u8]) {
    let mut file = File::create(path);
    file.write(contents);
}