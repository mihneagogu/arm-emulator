use crate::emulator::em_utilities as util;
use util::*;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive, Debug)]
pub enum ShiftOp {
    LSL = 0,
    LSR = 1,
    ASR = 2,
    ROR = 3,
}


fn shifted_reg_m_bits(bits: u32) -> u32 {
    process_mask(bits, bp32![0], bp32![3])
}

fn shift_mode_bit(bits: u32) -> bool {
    bit_mask(bits, bp32![4]) != 0
}

fn shift_type_bits(bits: u32) -> u32 {
    mask![bits, 5, 6]
}

fn shift_register_bits(bits: u32) -> u32 {
    mask![bits, 8, 11]
}

fn shift_constant_bits(bits: u32) -> u32 { mask![bits, 7, 11] }


fn execute_shift(operand: u32, shift_amount: u32, shift_opcode: ShiftOp,
                 c_bit: &mut u8) -> u32 {
    let mut result: u32;
    let mut cbit: u64;
    match shift_opcode {
        ShiftOp::LSL => {
            result = operand << shift_amount;
            // CPSR c bit is set to bit 32 after lsl (carry out)
            cbit = (((operand as u64) << shift_amount as u64) >> 32) & 1;
        }
        ShiftOp::LSR => {
            result = operand >> shift_amount;
            cbit = ( (operand >> (shift_amount - 1)) & 1 ) as u64;
        }
        ShiftOp::ASR => {
            result = arithmetic_shift_right(operand, shift_amount);
            cbit = ( (operand >> (shift_amount - 1)) & 1 ) as u64;
        }
        ShiftOp::ROR => {
            result = rotate_right(operand, shift_amount);
            cbit = ( (operand >> (shift_amount - 1)) & 1 ) as u64;
        }
    };
    *c_bit = cbit as u8;
    result
}

pub fn reg_offset_shift(cpu: &CpuState, instr: &Instruction, c_bit: &mut u8) -> u32 {
    let mut result: u32 = 0;
    let bits = instr.code;
    let reg_contents: u32 = cpu.registers[shifted_reg_m_bits(bits) as usize];

    if shift_mode_bit(bits) {
        let lower_byte: u8 = cpu.registers[shift_register_bits(bits) as usize] as u8;
        let shift_type = shift_type_bits(bits);
        let shift_type = FromPrimitive::from_u32(shift_type).unwrap();
        result = execute_shift(reg_contents, lower_byte as u32, shift_type, c_bit);
    } else {
        let shift_type = shift_type_bits(bits);
        let shift_type = FromPrimitive::from_u32(shift_type).unwrap();
        result = execute_shift(reg_contents, shift_constant_bits(bits), shift_type, c_bit);
    }

    result
}

pub fn rotate_right(operand: u32, rotate_amount: u32) -> u32 {
    let mut result: u32 = operand >> rotate_amount;
    result |= operand << (32 - rotate_amount);

    result
}

pub fn arithmetic_shift_right(operand: u32, shift_amount: u32) -> u32 {
    let mut result: u32 = 0;
    if ((1 << 31) & operand) != 0 {
        // MSB is 1
        result = operand >> shift_amount;
        for i in 0..shift_amount {
            result |= 1 << (31 - 1);
        }
    } else {
        result = operand >> shift_amount;
    }
    result
}



