pub struct StatusFlags {
    pub n: bool,
    pub z: bool,
    pub v: bool,
    pub c: bool,
}

pub struct Registers {
    regs: [u16; 8]
}

impl Registers {
    pub fn new() -> Self {
        Self { regs: [0; 8] }
    }

    pub fn read(&self, reg: u8) -> u16 {
        if reg == 0 { 0 } else { self.regs[reg as usize] }
    }

    pub fn write(&mut self, reg: u8, val: u16) -> () {
        self.regs[reg as usize] = val;
    }

    pub fn pc(&self) -> u16 {
        self.regs[7]
    }

    pub fn incrememnt_pc(&mut self) {
        self.regs[7] += 1;
    }

    pub fn set_pc(&mut self, val: u16) {
        self.regs[7] = val;
    }
}