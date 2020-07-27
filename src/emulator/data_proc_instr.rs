use crate::emulator::em_utilities as util;
use util::*;

/// Returns whether the immediate is enabled for the given instruction
fn immediate_enabled(bits: u32) -> bool {
    bit_mask(bits, BitPos32::from_u8(25)) != 0
}

fn cpsr_enabled(bits: u32) -> bool {
    bit_mask(bits, BitPos32::from_u8(20)) != 0
}

fn opcode_bits(bits: u32) -> u32 {
    process_mask(bits, bp32(21), bp32(24))
}

fn dest_reg(bits: u32) -> u32 {
    process_mask(bits, bp32(12), bp32(15))
}

fn operand1_reg_bits(bits: u32) -> u32 {
    process_mask(bits, bp32![16], bp32![19])
}

fn operand2_reg_bits(bits: u32) -> u32 {
    process_mask(bits, bp32![0], bp32![11])
}

/// Executes a data processing instruction
fn execute_data_processing_instr(instr: &Instruction, cpu: &mut CpuState) {
    let bits = instr.code;
    let mut operand1: u32 = cpu.registers[operand1_reg_bits(bits) as usize];
    let mut operand2: u32 = operand2_reg_bits(bits);

    // will be the computed result that is written into the dest_register
    let mut result: u32 = 0;
    // if write result is 0 then the result is NOT written to the dest_register
    let mut write_result: u8 = 0;
    // c_bit is 1 if 1 is to be written to the C bit fo CPSR
    let mut c_bit: u8 = 0;

    // Compute operand2
    if immediate_enabled(bits) {
        operand2 = process_mask(bits, bp32![0], bp32![7]);
        // TODO:  operand_2 = rotate_right(operand_2, process_mask(instr->code, 8, 11) * 2);
        c_bit = ((operand2 >> (process_mask(bits, bp32![8], bp32![11]) * 2)) as u8) & 1;
    } else {
        // TODO: operand_2 = reg_offset_shift(cpu_state, instr, &c_bit);
    }
}
