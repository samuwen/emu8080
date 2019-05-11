use std::fmt;

pub struct Opcode {
    pub code: u8,
    pub operation_name: String,
    pub next_bytes: u16,
    pub cycles: u8,
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Opcode  {:4} : {:2x} | {:4x}",
            self.operation_name, self.code, self.next_bytes
        )
    }
}

impl Opcode {
    pub fn new(val: u8) -> Opcode {
        Opcode {
            code: val,
            operation_name: opcode_name(val).0,
            next_bytes: 0,
            cycles: opcode_name(val).1,
        }
    }
}

fn opcode_name(val: u8) -> (String, u8) {
    let s = match val {
        0x00 | 0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 | 0xCB | 0xD9 | 0xDD | 0xED
        | 0xFD => ("NOP", 4),
        0x01 | 0x11 | 0x21 | 0x31 => ("LXI", 10),
        0x02 | 0x12 => ("STAX", 7),
        0x03 | 0x13 | 0x23 | 0x33 => ("INX", 5),
        0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => ("INR", 5),
        0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => ("DCR", 5),
        0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => ("MVI", 10),
        0x07 => ("RLC", 4),
        0x09 | 0x19 | 0x29 | 0x39 => ("DAD", 10),
        0x0A | 0x1A => ("LDAX", 7),
        0x0B | 0x1B | 0x2B | 0x3B => ("DCX", 5),
        0x0F => ("RRC", 4),
        0x17 => ("RAL", 4),
        0x1F => ("RAR", 4),
        0x22 => ("SHLD", 16),
        0x27 => ("DAA", 4),
        0x2A => ("LHLD", 16),
        0x2F => ("CMA", 4),
        0x32 => ("STA", 13),
        0x37 => ("STC", 4),
        0x3A => ("LDA", 13),
        0x3F => ("CMC", 4),
        0x40...0x7F => ("MOV", 6),
        0x80...0x87 => ("ADD", 4),
        0x88...0x8F => ("ADC", 4),
        0x90...0x97 => ("SUB", 4),
        0x98...0x9F => ("SBB", 4),
        0xA0...0xA7 => ("ANA", 4),
        0xA8...0xAF => ("XRA", 4),
        0xB0...0xB7 => ("ORA", 4),
        0xB8...0xBF => ("CMP", 4),
        0xC1 | 0xD1 | 0xE1 | 0xF1 => ("POP", 10),
        0xC2 => ("JNZ", 10),
        0xC3 => ("JMP", 10),
        0xC4 => ("CNZ", 17),
        0xC5 | 0xD5 | 0xE5 | 0xF5 => ("PUSH", 10),
        0xC6 => ("ADI", 7),
        0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => ("RST", 11),
        0xC8 => ("RZ", 11),
        0xC9 => ("RET", 10),
        0xCA => ("JZ", 10),
        0xCC => ("CZ", 17),
        0xCD => ("CALL", 17),
        0xCE => ("ACI", 7),
        0xD0 => ("RNC", 11),
        0xD2 => ("JNC", 10),
        0xD3 => ("OUT", 10),
        0xD4 => ("CNC", 17),
        0xD6 => ("SUI", 7),
        0xD8 => ("RC", 11),
        0xDA => ("JC", 10),
        0xDB => ("IN", 10),
        0xDC => ("CC", 17),
        0xE0 => ("RPO", 11),
        0xE2 => ("JPO", 10),
        0xE3 => ("XTHL", 18),
        0xE4 => ("CPO", 17),
        0xE6 => ("ANI", 7),
        0xE8 => ("RPE", 11),
        0xE9 => ("PCHL", 5),
        0xEA => ("JPE", 10),
        0xEB => ("XCHG", 4),
        0xEC => ("CPE", 17),
        0xEE => ("XRI", 7),
        0xF0 => ("RP", 11),
        0xF2 => ("JP", 10),
        0xF3 => ("DI", 4),
        0xF4 => ("CP", 17),
        0xF6 => ("ORI", 7),
        0xF8 => ("RM", 11),
        0xF9 => ("SPHL", 5),
        0xFA => ("JM", 10),
        0xFB => ("EI", 4),
        0xFC => ("CM", 17),
        0xFE => ("CPI", 7),
        _ => panic!("How is babby formed {:x}", val),
    };
    (String::from(s.0), s.1)
}
