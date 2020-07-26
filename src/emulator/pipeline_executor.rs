mod em_utilities;
use em_utilities::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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

    // Alternatively could have made a Hashmap containg the instruciton types and closures for
    // execution
    // Managed to store them using Rc<RefCell<Box<dyn FnOnce()>>> for the functions
    // but getting the closures to execute was harder
    // This solution works just fine albeit less advanced and more C-like
    match instr.instruction_type {
        InstructionType::BRANCH => {
            // check whether branch succeeded
        }
        InstructionType::DATA_PROCESS => {
            // execute data processing instruction
            pipe.clear_executing();
        },
        InstructionType::MULTIPLTY => {
            // execute multiply instruction 
            pipe.clear_executing();
        },
        InstructionType::SINGLE_DATA_TRANSFER => {
            // execute multiply instruction 
            pipe.clear_executing();
        },

    }
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
