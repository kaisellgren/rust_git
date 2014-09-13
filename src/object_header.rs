use object_type::ObjectType;

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
