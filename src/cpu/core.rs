use crate::cpu::registers::{Registers, StatusFlags};
use crate::cpu::memory::Memory;
use crate::cpu::instruction::{decode, Instruction, Type1, Type2, Type3};
use crate::cpu::alu::{execute, AluOp};

pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory,
    pub flags: StatusFlags,
    cycle: Cycle,
    ir: Option<Instruction>,
}

enum Cycle {
    Fetch,
    Execute,
    Memory,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(),
            flags: StatusFlags::default(),
            cycle: Cycle::Fetch,
            ir: None
        }
    }

    pub fn step (&mut self) {
        match self.cycle {
            Cycle::Fetch => self.fetch_cycle(),
            Cycle::Execute => self.execute_cycle(),
            Cycle::Memory => self.memory_cycle(),
        }

        self.cycle = match self.cycle {
            Cycle::Fetch => Cycle::Execute,
            Cycle::Execute => {
                match &self.ir {
                    Some(Instruction::Type1(t1)) if t1.op == AluOp::LdSt => Cycle::Memory,
                    Some(Instruction::Type2(t2)) if t2.op == AluOp::LdSt => Cycle::Memory,
                    _ => Cycle::Fetch,
                }
            },
            Cycle::Memory => Cycle::Fetch,
        }
    }

    fn fetch_cycle(&mut self) {}
    fn execute_cycle(&mut self) {}
    fn memory_cycle(&mut self) {}

    fn execute_type1(&mut self, instr: &Type1) {}
    fn execute_type2(&mut self, instr: &Type2) {}
    fn execute_type3(&mut self, instr: &Type3) {}

}