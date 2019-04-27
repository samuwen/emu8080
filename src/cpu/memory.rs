#[derive(Clone, Copy)]
pub struct Memory {
    pub ram: [u8; 0x10000],
}

impl Default for Memory {
    fn default() -> Self {
        Memory { ram: [0; 0x10000] }
    }
}
