use std::str;
use object_id;
use object_id::ObjectId;
use error::GitError;
use error::GitError::InvalidString;

pub struct Reader<'a> {
    position: uint,
    data: &'a [u8]
}

impl<'a> Reader<'a> {
    pub fn from_data(bytes: &'a[u8]) -> Reader<'a> {
        Reader {
            position: 0,
            data: bytes
        }
    }

    pub fn take(&mut self, length: uint) -> &'a [u8] {
        self.skip(length);
        self.data.slice(self.position - length, self.position)
    }

    pub fn take_while(&mut self, predicate: |&u8| -> bool) -> &'a [u8] {
        let mut next_position = self.position;

        while predicate(&(self.data[next_position])) {
            next_position += 1;
        }

        let length = next_position - self.position;
        self.take(length)
    }

    pub fn take_string_while(&mut self, predicate: |&u8| -> bool) -> Result<&'a str, GitError> {
        str::from_utf8(self.take_while(predicate)).ok_or(InvalidString)
    }

    pub fn take_string(&mut self, length: uint) -> Result<&'a str, GitError> {
        str::from_utf8(self.take(length)).ok_or(InvalidString)
    }

    pub fn take_string_based_object_id(&mut self) -> Result<ObjectId, GitError> {
        self.take_string(40).and_then(ObjectId::from_string)
    }

    pub fn take_object_id(&mut self) -> ObjectId {
        ObjectId::from_bytes(self.take(object_id::RAW_SIZE))
    }

    pub fn get(&self, length: uint) -> &'a [u8] {
        self.data.slice(self.position, self.position + length)
    }

    pub fn get_rest(&self) -> &'a [u8] {
        self.data.slice_from(self.position)
    }

    pub fn get_rest_as_string(&self) -> Result<&'a str, GitError> {
        str::from_utf8(self.get_rest()).ok_or(InvalidString)
    }

    pub fn skip(&mut self, length: uint) {
        self.position += length;
    }

    pub fn back(&mut self, length: uint) {
        self.position -= length;
    }
}