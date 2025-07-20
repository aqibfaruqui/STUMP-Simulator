pub struct Memory {
    mem: [u16; 0xFFFF]
}

impl Memory {
    pub fn new() -> Self {
        Self { mem: [0; 0xFFFF] }
    }

    pub fn read(&self, addr: u16) -> u16 {
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u16) {
        self.mem[addr as usize] = val;
    }
}