use error::GitError;
use error::GitError::InvalidString;
use result::EmptyResult;
use std::io::fs::File;
use std::str;
use config::reader;
use config::writer;
use std::collections::hash_map::HashMap;
use std::io::FileMode;
use std::io::FileAccess;

pub type ConfigEntries = HashMap<String, Vec<KeyValue>>;

pub struct KeyValue(pub String, pub String);

pub struct Config {
    local:  Path,
    global: Path,
    system: Path,
}

impl Config {
    pub fn from_path(path: &Path) -> Config {
        Config {
            local:  path.join("config").clone(),
            global: path.join("config").clone(),
            system: path.join("config").clone(),
        }
    }

    pub fn set_local(&self, category: &str, key: &str, value: &str) -> EmptyResult {
        let entries = self.read(&self.local);

        Ok(())
    }

    pub fn read(&self, path: &Path) -> Result<ConfigEntries, GitError> {
        let data = try!(File::open(path).read_to_end());
        let mut contents = try!(str::from_utf8(data.as_slice()).ok_or(InvalidString));

        Ok(reader::read(contents.as_slice()))
    }

    pub fn write(&self, path: &Path, entries: ConfigEntries) {
        File::open_mode(path, FileMode::Truncate, FileAccess::Write)
            .write(writer::write(entries).as_slice()).unwrap(); // todo: error
    }
}

#[test]
fn parse_fixture_config() {
    let c = Config::from_path(&Path::new("resources/tests"));
    let entries = c.read(&c.local).unwrap();

    let has_categories = ["core", "remote \"origin\"", "branch \"master\""];
    assert!(entries.keys().count() == 3);

    for cat in has_categories.iter() {
        assert!(entries.keys().any(|c| c.as_slice() == *cat));
    }

    let mut correct = HashMap::new();

    correct.insert("core".into_string(), vec![
      KeyValue("repositoryformatversion".into_string(), "0".into_string())
    ]);

    c.write(&Path::new("resources/tests/configoutput"), entries);
}