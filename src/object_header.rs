use object_type::ObjectType;
use object_type;
use reader::Reader;
use std::from_str::FromStr;

#[deriving(PartialEq, Show)]
pub struct ObjectHeader {
    pub typ: ObjectType,
    pub length: uint
}

impl ObjectHeader {
    pub fn encode(&self) -> Vec<u8> {
        let mut buff = Vec::new();

        buff.push_all(format!("{} ", self.typ.to_text()).into_bytes().as_slice());
        buff.push_all(format!("{}", self.length).into_bytes().as_slice());
        buff.push(0x00);

        buff
    }
}

pub fn decode(bytes: &[u8]) -> ObjectHeader {
    let mut reader = Reader::from_data(bytes);

    let typ = object_type::from_text(reader.take_string_while(|c| *c != 32));

    reader.skip(1);

    let length: uint = FromStr::from_str(reader.take_string_while(|c| *c != 0)).unwrap();

    ObjectHeader {
        typ: typ,
        length: length
    }
}