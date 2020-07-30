use crate::emulator::{barrel_shifter as shifter, em_utilities as util};
use shifter::*;
use util::*;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, FromPrimitive)]
enum DataProcOpcode {
    AND = 0,
    EOR = 1,
    SUB = 2,
    RSB = 3,
    ADD = 4,
    TST = 8,
    TEQ = 9,
    CMP = 10,
    ORR = 12,
    MOV = 13,
}

macro_rules! immediate_enabled {
    ($bits:expr) => {
        bit_mask($bits, bp32![25]) != 0
    };
}

macro_rules! cpsr_enabled {
    ($bits:expr) => {
        bit_mask($bits, bp32![20]) != 0
    };
}

macro_rules! dest_reg {
    ($bits:expr) => {
        mask![$bits, 12, 15]
    };
}

macro_rules! operand1_reg_bits {
    ($bits:expr) => {
        mask![$bits, 16, 19];
    };
}

macro_rules! operand2_reg_bits {
    ($bits:expr) => {
        mask![$bits, 0, 11]
    };
}

/// Gets the opcode bits
macro_rules! opcode_bits {
    ($bits:expr) => {
        mask![$bits, 21, 24]
    };
}

/// Executes a data processing instruction
pub fn execute_data_processing_instr(instr: &Instruction, cpu: &mut CpuState) {
    let bits = instr.code;
    let operand1: u32 = cpu.registers[operand1_reg_bits![bits] as usize];
    let mut operand2: u32 = operand2_reg_bits![bits];

    // will be the computed result that is written into the dest_register
    let mut result: u32 = 0;
    // if write result is 0 then the result is NOT written to the dest_register
    let mut write_result: u8 = 1;
    // c_bit is 1 if 1 is to be written to the C bit fo CPSR
    let mut c_bit: u8 = 0;

    // Compute operand2
    if immediate_enabled![bits] {
        operand2 = process_mask(bits, bp32![0], bp32![7]);
        operand2 = rotate_right(operand2, process_mask(bits, bp32![8], bp32![11]) * 2);
        c_bit = ((operand2 >> (process_mask(bits, bp32![8], bp32![11]) * 2)) as u8) & 1;
    } else {
        operand2 = reg_offset_shift(cpu, &instr, &mut c_bit);
    }

    let opcode = opcode_bits![bits];
    let opcode = FromPrimitive::from_u32(opcode).unwrap();

    match opcode {
        DataProcOpcode::AND => {
            result = operand1 & operand2;
        }
        DataProcOpcode::EOR => {
            result = operand1 ^ operand2;
        }
        DataProcOpcode::SUB => {
            result = operand1 - operand2;
            // set c_bit if op2 > op1 (overflow)
            c_bit = if operand2 > operand1 { 0 } else { 1 };
        }
        DataProcOpcode::RSB => {
            result = operand2 - operand1;
            c_bit = if operand1 > operand2 { 0 } else { 1 };
        }
        DataProcOpcode::ADD => {
            result = operand1 + operand2;
            // set c_bit if it overflows
            let overflow_check: u64 = (operand1 as u64) + (operand2 as u64);
            c_bit = if overflow_check >= ((1 as u64) << 32) {
                1
            } else {
                0
            };
        }
        DataProcOpcode::TST => {
            result = operand1 & operand2;
            write_result = 0;
        }
        DataProcOpcode::TEQ => {
            result = operand1 ^ operand2;
            write_result = 0;
        }
        DataProcOpcode::CMP => {
            write_result = 0;
            if operand2 > operand1 {
                // overflows so it wraps
                result = operand2 - operand1;
                c_bit = 0;
            }
            else {
                c_bit = 1;
                result = operand1 - operand2;
                write_result = 0;
                c_bit = if operand2 > operand1 { 0 } else { 1 };
            }
        }
        DataProcOpcode::ORR => {
            result = operand1 | operand2;
        }
        DataProcOpcode::MOV => {
            result = operand2;
        }
    }

    if write_result != 0 {
        cpu.registers[dest_reg![bits] as usize] = result;
    }

    // CPSR flags

    if cpsr_enabled![bits] {
        // C bit (bit 29 CPSR) - set to c_bit which is determined by the opcode:
        cpu.set_CPSR_flag(Flag::C, c_bit != 0);

        //Z bit (bit 30 of CPSR) - Z is 1 iff result == 0:
        if result == 0 {
            cpu.set_CPSR_flag(Flag::Z, true);
        } else {
            cpu.set_CPSR_flag(Flag::Z, false);
        }

        cpu.set_CPSR_flag(Flag::N, mask![result, 31]);
    }
}
