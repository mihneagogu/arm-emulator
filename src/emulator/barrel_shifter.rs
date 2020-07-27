use crate::emulator::em_utilities as util;
use util::*;

pub enum ShiftOp {
    LSL,
    LSR,
    ASR,
    ROR,
}

fn shifted_reg_m_bits(bits: u32) -> u32 {
    process_mask(bits, bp32![0], bp32![3])
}

fn shift_mode_bit(bits: u32) -> bool {
    bit_mask(bits, bp32![4]) != 0
}

macro_rules! shift_register_bits {
    ($bits:expr) => {
        process_mask($bits, bp32![8], bp32![11])
    }
}

fn execute_shift(operand: u32, shift_amount: u32, shift_opcode: ShiftOp,
    c_bit: &mut u8) -> u32 {
    let mut result: u32;
    
    match shift_opcode {
        ShiftOp::LSL => {
            result = operand << shift_amount;
            // CPSR c bit is set to bit 32 after lsl (carry out)
            let cbit = ( ((operand as u64) << shift_amount) >> 32 ) & 1;
            *c_bit = cbit as u8;
        }
        ShiftOp::LSR => {
            result = operand >> shift_amount;
            let cbit = (operand >> (shift_amount - 1)) & 1;
            *c_bit = cbit as u8;
        }
        ShiftOp::ASR => {
            // TODO: result = arithmetic_shift_right(operand, shift_amount);
            ////bit (shift_amount - 1) is the carry out
            //*c_bit = (operand >> (shift_amount - 1)) & 1;
        }
        ShiftOp::ROR => {
            // TODO: result = rotate_right(operand, shift_amount);
            ////bit (shift_amount - 1) is the carry out
            //*c_bit = (operand >> (shift_amount - 1)) & 1;
        }
    };
    0
}

pub fn reg_offset_shift(cpu: &CpuState, instr: &mut Instruction, c_bit: u8) -> u32 {
    let mut result: u32 = 0;
    let bits = instr.code;
    let reg_contents: u32 = cpu.registers[shifted_reg_m_bits(bits) as usize];

    if shift_mode_bit(bits) {
        let lower_byte: u8 = cpu.registers[shift_register_bits![bits] as usize] as u8;
        // TODO: result = execute_shift(reg_contents, lower_byte, SHIFT_TYPE_BITS(instr->code), c_bit);
    } else {
        // TODO: result = execute_shift(reg_contents, SHIFT_CONSTANT_BITS(instr->code), 
            //SHIFT_TYPE_BITS(instr->code), c_bit); 
    }

    result
}
