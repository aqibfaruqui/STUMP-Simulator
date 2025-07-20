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
    op: AluOp,
    update_flags: bool,
    st_cc: bool,
    dest: u8,
    reg_a: u8,
    reg_b: u8,
    shift: u8,
}

pub struct Type2 {
    op: AluOp,
    update_flags: bool,
    st_cc: bool,
    dest: u8,
    reg_a: u8,
    immed: u8,
}

pub struct Type3 {
    cond: u8,
    offset: u8,
}

pub enum Instrucion {
    Type1(Type1),
    Type2(Type2),
    Type3(Type3),
}

pub fn decode(instr: u16) -> Instrucion {
    let op_bits = instr.bits(15, 13) as u8;
    
    if op_bits == 7 {
        return Instrucion::Type3(Type3 {
            cond: instr.bits(11, 8) as u8;
            offset: instr.bits(7, 0) as u8;
        })
    }

    let is_type2 = instr.bit(12);
    let st_cc = instr.bit(11);
    let dest = instr.bits(10, 8) as u8;
    let reg_a = instr.bits(7, 5) as u8;
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
        st_cc;
    }
    
    if is_type2 {
        return Instrucion::Type2(Type2 {
            op,
            update_flags,
            st_cc,
            dest,
            reg_a,
            immed: instr.bits(4, 0) as u8,
        })
    } else {
        return Instrucion::Type1(Type1 {
            op,
            update_flags,
            st_cc,
            dest,
            reg_a,
            reg_b: instr.bits(4, 2) as u8,
            shift: instr.bits(1, 0) as u8,
        })
    }
}