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
        0x00 | 0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 | 0xCB | 0xD9 | 0xDD | 0xED
        | 0xFD => String::from("NOP"),
        0x01 | 0x11 | 0x21 | 0x31 => String::from("LXI"),
        0x02 | 0x12 => String::from("STAX"),
        0x03 | 0x13 | 0x23 | 0x33 => String::from("INX"),
        0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => String::from("INR"),
        0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => String::from("DCR"),
        0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => String::from("MVI"),
        0x07 => String::from("RLC"),
        0x09 | 0x19 | 0x29 | 0x39 => String::from("DAD"),
        0x0A | 0x1A => String::from("LDAX"),
        0x0F => String::from("RRC"),
        0x17 => String::from("RAL"),
        0x1F => String::from("RAR"),
        0x22 => String::from("SHLD"),
        0x27 => String::from("DAA"),
        0x2A => String::from("LHLD"),
        0x80...0x87 => String::from("ADD"),
        0x88...0x8F => String::from("ADC"),
        0x90...0x97 => String::from("SUB"),
        0x98...0x9F => String::from("SBB"),
        0xA0...0xA7 => String::from("ANA"),
        0xA8...0xAF => String::from("XRA"),
        0xB0...0xB7 => String::from("ORA"),
        0xB8...0xBF => String::from("CMP"),
        _ => panic!("How is babby formed"),
    }
}
