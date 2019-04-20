#[derive(Clone, Copy, Default, Debug)]
// Tuple struct
pub struct Register(u8);

impl From<u16> for Register {
    fn from(t: u16) -> Self {
        Register(t as u8)
    }
}

impl From<u8> for Register {
    fn from(t: u8) -> Self {
        Register(t)
    }
}

impl From<usize> for Register {
    fn from(t: usize) -> Self {
        Register(t as u8)
    }
}

impl From<&Register> for u8 {
    fn from(t: &Register) -> Self {
        t.0
    }
}

impl From<Register> for u16 {
    fn from(t: Register) -> Self {
        t.0 as u16
    }
}

impl PartialEq<u8> for Register {
    fn eq(&self, other: &u8) -> bool {
        &self.0 == other
    }
}
