use stack_vm::{FromByteCode, ToByteCode};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Imm(i64),
    Label(String),
}

const SIG_IMM: u8 = 0;
const SIG_LABEL: u8 = 1;

impl Operand {
    pub fn sig(&self) -> u8 {
        match self {
            Operand::Imm(_) => SIG_IMM,
            Operand::Label(_) => SIG_LABEL,
        }
    }

    pub fn imm(&self) -> i64 {
        let Operand::Imm(imm) = self else {
            panic!("oprand is not imm")
        };
        *imm
    }

    pub fn label(&self) -> String {
        let Operand::Label(label) = self else {
            panic!("oprand is not label")
        };
        label.to_string()
    }
}

impl ToByteCode for Operand {
    fn to_byte_code(&self, mut reader: &mut dyn Write) {
        rmp::encode::write_u8(&mut reader, self.sig()).unwrap();
        match self {
            Operand::Imm(imm) => {
                rmp::encode::write_sint(&mut reader, *imm).unwrap();
            }
            Operand::Label(label) => {
                rmp::encode::write_str(&mut reader, label.as_str()).unwrap();
            }
        };
    }
}

impl FromByteCode for Operand {
    fn from_byte_code(mut reader: &mut dyn Read) -> Self {
        let sig = rmp::decode::read_u8(&mut reader).unwrap();
        match sig {
            SIG_IMM => {
                let value = rmp::decode::read_int(&mut reader).unwrap();
                Operand::Imm(value)
            }
            SIG_LABEL => {
                let mut buf: Vec<u8> = vec![0; 100];
                let value = rmp::decode::read_str(&mut reader, &mut buf).unwrap();
                Operand::Label(value.to_string())
            }
            _ => panic!("unknown sig"),
        }
    }
}
