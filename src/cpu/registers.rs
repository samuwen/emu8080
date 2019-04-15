pub struct Registers {
    pub pc: u16,
    pub register: [u8; 7],
}

impl Registers {
    pub fn get_value(&self, register: &str) -> u8 {
        let index = self.get_index(register);
        self.register[index]
    }

    pub fn set_value(&mut self, register: &str, val: u8) {
        let index = self.get_index(register);
        self.register[index] = val;
    }

    fn get_index(&self, register: &str) -> usize {
        match register {
            "B" => 0,
            "C" => 1,
            "D" => 2,
            "E" => 3,
            "H" => 4,
            "L" => 5,
            "A" => 6,
            _ => panic!("Invalid index"),
        }
    }
}
