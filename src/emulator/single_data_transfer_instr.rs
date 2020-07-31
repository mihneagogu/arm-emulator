use crate::emulator::barrel_shifter::reg_offset_shift;
use crate::emulator::em_utilities as util;
use util::*;

macro_rules! immediate_bit {
    ($bits:expr) => {
        mask![$bits, 25]
    };
}

macro_rules! indexing_bit {
    ($bits:expr) => {
        mask![$bits, 24]
    };
}

macro_rules! up_bit {
    ($bits:expr) => {
        mask![$bits, 23]
    };
}

macro_rules! transfer_type_bit {
    ($bits:expr) => {
        mask![$bits, 20]
    };
}

macro_rules! offset_bits {
    ($bits:expr) => {
        mask![$bits, 0, 11] as u16
    };
}

macro_rules! base_reg_bits {
    ($bits:expr) => {
        mask![$bits, 16, 19] as usize
    };
}

macro_rules! transfer_reg_bits {
    ($bits:expr) => {
        mask![$bits, 12, 15] as usize
    };
}

/// Computes the address given to an offset of an SDT instruction
fn compute_address(cpu: &mut CpuState, instr: &Instruction, offset: u32) -> u32 {
    let address: u32;
    let bits = instr.code;
    let base_reg_val: u32 = cpu.registers[base_reg_bits![bits]];

    if up_bit![bits] {
        if indexing_bit![bits] {
            address = base_reg_val + offset;
        } else {
            address = base_reg_val;
            cpu.registers[base_reg_bits![bits]] += offset;
        }
    } else {
        if indexing_bit![bits] {
            address = base_reg_val - offset;
        } else {
            address = base_reg_val;
            cpu.registers[base_reg_bits![bits]] -= offset;
        }
    }

    address
}

/// Computes the offset of an SDT instruction
fn compute_offset(cpu: &mut CpuState, instr: &Instruction) -> u16 {
    let offset: u16;
    let bits = instr.code;
    if immediate_bit![bits] {
        // Register shifted offset (as in data processing type instruction)
        let mut carry: u8 = 0;
        offset = reg_offset_shift(cpu, instr, &mut carry) as u16;
    } else {
        offset = offset_bits![bits];
    }
    offset
}

const NUM_REGISTERS: u32 = 17;
pub fn execute_single_data_instr(instr: &Instruction, cpu: &mut CpuState) {
    let bits = instr.code;
    assert!(transfer_reg_bits![bits] < NUM_REGISTERS as usize);
    let offset: u16 = compute_offset(cpu, instr);
    let address: u32 = compute_address(cpu, instr, offset as u32);

    if transfer_type_bit![bits] {
       let word: u32 = cpu.index_little_endian(address as usize);
        cpu.registers[transfer_reg_bits![bits]] = word;
    } else {
        let reg_val: u32 = cpu.registers[transfer_reg_bits![bits]];
        let address = address as usize;
        cpu.memory[address] = mask![reg_val, 0, 7] as u8;
        cpu.memory[address + 1] = mask![reg_val, 8, 15] as u8;
        cpu.memory[address + 2] = mask![reg_val, 16, 23] as u8;
        cpu.memory[address + 3] = mask![reg_val, 24, 31] as u8;
    }
}
