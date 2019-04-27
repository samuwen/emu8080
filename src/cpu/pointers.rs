use std::ops::{Add, AddAssign, BitAnd, SubAssign};

#[derive(Clone, Copy, Default, Debug)]
pub struct Pointer {
    x: u16,
}

impl From<u8> for Pointer {
    fn from(t: u8) -> Self {
        Pointer { x: t as u16 }
    }
}

impl From<u16> for Pointer {
    fn from(t: u16) -> Self {
        Pointer { x: t }
    }
}

impl From<Pointer> for usize {
    fn from(t: Pointer) -> usize {
        t.x as usize
    }
}

impl From<Pointer> for u16 {
    fn from(t: Pointer) -> u16 {
        t.x as u16
    }
}

impl From<Pointer> for u8 {
    fn from(t: Pointer) -> u8 {
        t.x as u8
    }
}

impl Add<u16> for Pointer {
    type Output = Pointer;

    fn add(self, other: u16) -> Pointer {
        Pointer { x: self.x + other }
    }
}

impl AddAssign<u16> for Pointer {
    fn add_assign(&mut self, other: u16) {
        *self = Pointer { x: self.x + other }
    }
}

impl SubAssign<u16> for Pointer {
    fn sub_assign(&mut self, other: u16) {
        *self = Pointer { x: self.x - other }
    }
}

impl PartialEq<u16> for Pointer {
    fn eq(&self, other: &u16) -> bool {
        &self.x == other
    }
}

impl BitAnd<u16> for Pointer {
    type Output = Pointer;

    fn bitand(self, other: u16) -> Pointer {
        Pointer { x: self.x & other }
    }
}
