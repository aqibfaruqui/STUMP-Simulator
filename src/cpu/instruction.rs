use intbits::Bits;
use crate::cpu::alu::AluOp;
use crate::cpu::registers::Registers;

/*
 * TODO: Move shift and branch condition matching to receiver of decode()
 */

pub enum Shift {
    NONE,               // No shift
    ASR,                // Arithmetic shift right
    ROR,                // Rotate right
    RRC,                // Rotate right through carry
}

pub enum BranchCondition {
    BAL = 0,                // Always
    BNV = 1,                // Never
    BHI = 2,                // C + Z = 0      Comparison: unsigned arithmetic
    BLS = 3,                // C + Z = 1      "
    BCC = 4,                // C = 0          Overflow test: unsigned arithmetic
    BCS = 5,                // C = 1          "
    BNE = 6,                // Z = 0          Zero test 
    BEQ = 7,                // Z = 1          "
    BVC = 8,                // V = 0          Overflow test: signed arithmetic
    BVS = 9,                // V = 1          "
    BPL = 10,               // N = 0              Comparison: signed arithmetic
    BMI = 11,               // N = 1              "
    BGE = 12,               // ¬N.V+N.¬V = 0      "
    BLT = 13,               // ¬N.V+N.¬V = 1      "
    BGT = 14,               // (¬N.V+N.¬V)+Z = 0  "
    BLE = 15,               // (¬N.V+N.¬V)+Z = 1  "
}

pub struct Type1 {
    pub op: AluOp,
    pub update_flags: bool,
    pub st_cc: bool,
    pub dest: u8,
    pub reg_a: u8,
    pub reg_b: u8,
    pub shift: u8,
}

pub struct Type2 {
    pub op: AluOp,
    pub update_flags: bool,
    pub st_cc: bool,
    pub dest: u8,
    pub reg_a: u8,
    pub immed: u8,
}

pub struct Type3 {
    pub cond: u8,
    pub offset: u8,
}

pub enum Instruction {
    Type1(Type1),
    Type2(Type2),
    Type3(Type3),
}

pub fn decode(instr: u16) -> Instruction {
    let op_bits = instr.bits(13..15) as u8;
    
    if op_bits == 7 {
        return Instruction::Type3(Type3 {
            cond: instr.bits(8..11) as u8,
            offset: instr.bits(0..7) as u8,
        })
    }

    let is_type2 = instr.bit(12);
    let st_cc = instr.bit(11);
    let dest = instr.bits(8..10) as u8;
    let reg_a = instr.bits(5..7) as u8;
    let op  = match op_bits {
        0 => AluOp::Add,
        1 => AluOp::Adc,
        2 => AluOp::Sub,
        3 => AluOp::Sbc,
        4 => AluOp::And,
        5 => AluOp::Or,
        6 => AluOp::LdSt,
        _ => unreachable!(),
    };

    let update_flags = if op == AluOp::LdSt {
        false
    } else {
        st_cc
    };
    
    if is_type2 {
        return Instruction::Type2(Type2 {
            op,
            update_flags,
            st_cc,
            dest,
            reg_a,
            immed: instr.bits(0..4) as u8,
        })
    } else {
        return Instruction::Type1(Type1 {
            op,
            update_flags,
            st_cc,
            dest,
            reg_a,
            reg_b: instr.bits(2..4) as u8,
            shift: instr.bits(0..1) as u8,
        })
    }
}