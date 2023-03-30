use std::io::{Read, Write};

use stack_vm::{FromByteCode, ToByteCode};

#[derive(Debug, Clone, PartialEq)]
pub struct Operand(pub i64);

impl ToByteCode for Operand {
    fn to_byte_code(&self, mut buf: &mut dyn Write) {
        rmp::encode::write_sint(&mut buf, self.0).unwrap();
    }
}

impl FromByteCode for Operand {
    fn from_byte_code(mut buf: &mut dyn Read) -> Self {
        let value = rmp::decode::read_int(&mut buf).unwrap();
        Operand(value)
    }
}
