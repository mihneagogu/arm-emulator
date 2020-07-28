use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::emulator::em_utilities as util;
use crate::emulator::branch_instr as branch;
use util::*;
use branch::execute_branch_instr;


/// Executes the emulator given the instruction vector
pub fn emulate(instructions: Vec<u32>) {
    let cpu = util::CpuState::init();
}

/// Executes the given instruction
fn execute_instr(instr: &mut Instruction, cpu: &mut CpuState, pipe: &mut Pipe) {
    let flag_code = process_mask(instr.code, BitPos32::from_u8(28), BitPos32::from_u8(31));
    let flag_code = FromPrimitive::from_u32(flag_code);
    match flag_code {
        Some(code) => {
            // Don't execute if the CPSR condition is failed
            if !cpu.check_CPSR_cond(code) {
                return;
            }
        }
        // We are assuming the binary file has correct instructions, so an instruction with a wrong
        // CPSR flag will panic
        None => {
            panic!("You gave me a wrong CPSR flag code, something is wrong with your binary file!")
        }
    };

    // Map from the type and the closure to be executed
    // A match statement would have been simpler, but did this because it's more advanced
    // and uses Rust's abstractions pretty well
    let mut executors: HashMap<InstructionType, fn(&mut Instruction, &mut Pipe, &mut CpuState) -> ()> = HashMap::new();

    executors.insert(
        InstructionType::DATA_PROCESS,
        |instr: &mut Instruction, pipe: &mut Pipe, cpu: &mut CpuState| {
            // execute data process instruction
            pipe.clear_executing();
        },
    );

    executors.insert(
        InstructionType::BRANCH,
        |instr: &mut Instruction, pipe: &mut Pipe, cpu: &mut CpuState| {
            // Check whether branch instruction succeeded
        },
    );

    executors.insert(
        InstructionType::SINGLE_DATA_TRANSFER,
        |instr: &mut Instruction, pipe: &mut Pipe, cpu: &mut CpuState| {
            // execute single data transfer instruction
            pipe.clear_executing();
        },
    );

    executors.insert(
        InstructionType::MULTIPLTY,
        |instr: &mut Instruction, pipe: &mut Pipe, cpu: &mut CpuState| {
            // execute multiply instruction
            pipe.clear_executing();
        },
    );

    executors
        .get(&instr.instruction_type)
        .map(|execute| execute(instr, pipe, cpu));
    //executors.get(&instr.instruction_type).map(|fun| { fun(pipe) });
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


pub fn start_pipeline(cpu: &mut CpuState) {
    let mut pipe = Pipe::init(cpu);
    start_pipeline_helper(cpu, &mut pipe);
}

fn start_pipeline_helper(cpu: &mut CpuState, pipe: &mut Pipe) {

}
