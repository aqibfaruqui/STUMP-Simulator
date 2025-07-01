use crate::cpu::registers::StatusFlags;

#[derive(PartialEq)]
pub enum AluOp {
    Add,
    Adc,
    Sub,
    Sbc,
    And,
    Or,
    LdSt,
    Bcc,
}

pub fn execute(op: AluOp, a: u16, b:u16, carry: bool, csh: bool) -> (u16, StatusFlags) {
    let result = match op {
        AluOp::Add => a + b,
        AluOp::Adc => a + b + carry as u16,
        AluOp::Sub => a - b,
        AluOp::Sbc => a - b - carry as u16,
        AluOp::And => a & b,
        AluOp::Or => a | b,
        AluOp::LdSt => a + b,
        AluOp::Bcc => a + b,
    };
    
    let flags = StatusFlags {
        n: (result as i16) < 0,
        z: result == 0,
        v: match op {
            AluOp::Add | AluOp::Adc => {
                let a = a as i16;
                let b = b as i16;
                let r = result as i16;
                (a > 0 && b > 0 && r < 0) || (a < 0 && b < 0 && r > 0)
            },
            AluOp::Sub | AluOp::Sbc => {
                let a = a as i16;
                let b = b as i16;
                let r = result as i16;
                (a > 0 && b < 0 && r < 0) || (a < 0 && b > 0 && r > 0)
            },
            _ => false,
        },
        c: match op {
            AluOp::Add | AluOp::Adc => {
                let res = a as u32 + b as u32 + if op == AluOp::Adc { carry as u32 } else { 0 };
                res > u16::MAX as u32
            },
            AluOp::Sub | AluOp::Sbc => {
                let res = a as i32 - b as i32 - if op == AluOp::Sbc { carry as i32 } else { 0 };
                res >= 0
            },
            AluOp::And | AluOp::Or => csh,
            _ => false,
        } 
    };

    (result, flags)
}