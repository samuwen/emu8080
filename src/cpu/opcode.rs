use std::fmt;

pub struct Opcode {
    pub op: u8,
    pub name: String,
    pub assc_bytes: (u8, u8),
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Opcode  {:2x}:{:8} |{:02x}{:02x}",
            self.op, self.name, self.assc_bytes.0, self.assc_bytes.1
        )
    }
}

impl Opcode {
    pub fn new() -> Opcode {
        Opcode {
            op: 0,
            name: String::from(""),
            assc_bytes: (0, 0),
        }
    }
}
