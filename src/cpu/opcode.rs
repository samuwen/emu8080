use std::fmt;

pub struct Opcode {
    pub code: u8,
    pub operation_name: String,
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opcode  {:2x}:{:4}", self.code, self.operation_name,)
    }
}

impl Opcode {
    pub fn new(val: u8) -> Opcode {
        Opcode {
            code: val,
            operation_name: opcode_name(val),
        }
    }
}

fn opcode_name(val: u8) -> String {
    match val {
        0x80...0x87 => String::from("ADD"),
        0x88...0x8F => String::from("ADC"),
        _ => panic!("How is babby formed"),
    }
}
