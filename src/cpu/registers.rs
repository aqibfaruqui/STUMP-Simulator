pub struct StatusFlags {
    pub n: bool,                // (N)egative
    pub z: bool,                // (Z)ero
    pub v: bool,                // o(V)erflow
    pub c: bool,                // (C)arry
}

pub struct Registers {          // R0    => Always 0
    regs: [u16; 8]              // R1-R6 => General Use
}                               // R7    => PC

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