mod em_utilities;
use em_utilities::*;
use std::collections::HashMap;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Executes the emulator given the instruction vector
pub fn emulate(instructions: Vec<u32>) {
    let cpu = CpuState::init();
}

/// Executes the given instruction
fn execute_instr(instr: Instruction, cpu: &mut CpuState, pipe: &mut Pipe) {
    let flag_code = process_mask(instr.code, BitPos32::from_u8(28), BitPos32::from_u8(31));
    let flag_code = FromPrimitive::from_u32(flag_code);
    match flag_code {
        Some(code) => {
            // Don't execute if the CPSR condition is failed
            if !cpu.check_CPSR_cond(code) {
                return;
            }
        }
        None => return, // Exit, must have gotten a wrong bit-mask, can be unreachable!()
    };

    // Map for executing jj
    let mut executors: HashMap<InstructionType, Box<dyn FnOnce()>> = HashMap::new();
    executors.insert(InstructionType::BRANCH, Box::new( || {
   
    }));


    executors.insert(InstructionType::MULTIPLTY, Box::new( || {
   
    }));

}

/// Helper function that helps with checking which instruction type
/// the given instruction is
fn instruction_condition(bits: u32, start: u8, end: u8, target: u32) -> bool {
    process_mask(bits, BitPos32::from_u8(start), BitPos32::from_u8(end)) == target
}

/// Returns whether the given instruction is of type BRANCH
fn is_branch_instr(bits: u32) -> bool {
    // Bits 24-27 are 1010
    instruction_condition(bits, 24, 27, 10)
}

/// Returns whether the given instruction is of type MULTIPLY
fn is_multiply_instr(bits: u32) -> bool {
    // Bits 22-27 are all 9 and bits 4-7 are 1001
    instruction_condition(bits, 22, 27, 0) && instruction_condition(bits, 4, 7, 9)
}

/// Returns whether the given instruction is of type SINGLE_DATA_TRANSFER
fn is_single_data_transfer_instr(bits: u32) -> bool {
    // Bits 26-27 are 01
    instruction_condition(bits, 26, 27, 1)
}

pub fn decode_instruction(bits: u32) -> Instruction {
    let mut instruction_type;
    if is_branch_instr(bits) {
        instruction_type = InstructionType::BRANCH;
    } else if is_multiply_instr(bits) {
        instruction_type = InstructionType::MULTIPLTY;
    } else if is_single_data_transfer_instr(bits) {
        instruction_type = InstructionType::SINGLE_DATA_TRANSFER;
    } else {
        instruction_type = InstructionType::DATA_PROCESS;
    }
    Instruction {
        code: bits,
        instruction_type,
    }
}
