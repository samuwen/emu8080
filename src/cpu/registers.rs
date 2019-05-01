use super::Pointer;
use std::fmt;

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

impl From<Pointer> for Register {
    fn from(t: Pointer) -> Self {
        Register(u8::from(t))
    }
}

impl From<Register> for u8 {
    fn from(t: Register) -> Self {
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

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:x})", self.0)
    }
}
