use std::fmt;

pub struct Opcode {
    pub code: u8,
    pub operation_name: String,
    pub operation_register: String,
    pub assc_bytes: (u8, u8),
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Opcode  {:2x}:{:4} {:4} |{:02x}{:02x}",
            self.code,
            self.operation_name,
            self.operation_register,
            self.assc_bytes.0,
            self.assc_bytes.1
        )
    }
}

impl Opcode {
    pub fn new() -> Opcode {
        Opcode {
            code: 0,
            operation_name: String::from(""),
            operation_register: String::from(""),
            assc_bytes: (0, 0),
        }
    }

    pub fn set_operation_register(&mut self) {
        match self.code % 8 {
            0 => self.operation_register = String::from("B"),
            1 => self.operation_register = String::from("C"),
            2 => self.operation_register = String::from("D"),
            3 => self.operation_register = String::from("E"),
            4 => self.operation_register = String::from("H"),
            5 => self.operation_register = String::from("L"),
            6 => self.operation_register = String::from("M"),
            7 => self.operation_register = String::from("A"),
            _ => panic!("Non euclidean math detected"),
        }
    }
}
