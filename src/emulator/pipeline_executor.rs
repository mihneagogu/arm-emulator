use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::io;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::emulator::branch_instr as branch;
use crate::emulator::data_proc_instr as data_proc;
use crate::emulator::em_utilities as util;
use branch::execute_branch_instr;
use data_proc::execute_data_processing_instr;
use util::*;

/// Executes the emulator given the instruction vector
pub fn emulate(path: &str) -> Result<(), std::io::Error> {
    let mut cpu = util::CpuState::init(path)?;
    
    start_pipeline(&mut cpu);
    cpu.print_registers();
    Ok(())
}

/// Executes the given instruction
fn execute_instr(instr: &Instruction, cpu: &mut CpuState, pipe: &mut Pipe) -> bool {
    let flag_code = process_mask(instr.code, BitPos32::from_u8(28), BitPos32::from_u8(31));
    let flag_code = FromPrimitive::from_u32(flag_code);
    match flag_code {
        Some(code) => {
            // Don't execute if the CPSR condition is failed
            if !cpu.check_CPSR_cond(code) {
                return false;
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
    let mut executors: HashMap<
        InstructionType,
        fn(&Instruction, &mut Pipe, &mut CpuState) -> bool,
    > = HashMap::new();

    executors.insert(
        InstructionType::DATA_PROCESS,
        |instr: &Instruction, pipe: &mut Pipe, cpu: &mut CpuState| {
            // execute data process instruction
            execute_data_processing_instr(instr, cpu);
            pipe.clear_executing();
            true
        },
    );

    executors.insert(
        InstructionType::BRANCH,
        |instr: &Instruction, pipe: &mut Pipe, cpu: &mut CpuState| {
            // Check whether branch instruction succeeded
            execute_branch_instr(instr, cpu, pipe)
        },
    );

    executors.insert(
        InstructionType::SINGLE_DATA_TRANSFER,
        |instr: &Instruction, pipe: &mut Pipe, cpu: &mut CpuState| {
            // execute single data transfer instruction
            pipe.clear_executing();
            true
        },
    );

    executors.insert(
        InstructionType::MULTIPLTY,
        |instr: &Instruction, pipe: &mut Pipe, cpu: &mut CpuState| {
            // execute multiply instruction
            pipe.clear_executing();
            true
        },
    );

    // Safe to unwrap since we know we have the required functions in the map
    let executor = executors.get(&instr.instruction_type).unwrap();

    executor(instr, pipe, cpu)
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

pub fn decode_instruction(bits: u32) -> Rc<Instruction> {
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
    Rc::new(Instruction {
        code: bits,
        instruction_type,
    })
}

pub fn start_pipeline(cpu: &mut CpuState) {
    let mut pipe = Pipe::init(cpu);
    start_pipeline_helper(cpu, &mut pipe);
}

fn start_pipeline_helper(cpu: &mut CpuState, pipe: &mut Pipe) {
    // TODO: Switch recursive call to loop
    if pipe.fetching != 0 {
        // Set decoding to None and move the previous decoding value to executing
        let new_exec = pipe.decoding.take();
        pipe.executing = new_exec;
        pipe.decoding = Some(decode_instruction(pipe.fetching));
        let mut branch_succeeded = false;
        if let Some(instr) = &pipe.executing {
            let instr_type = instr.instruction_type;
            let succeeded = execute_instr(&Rc::clone(instr), cpu, pipe);
            if succeeded && (instr_type == InstructionType::BRANCH) {
                branch_succeeded = true;
            }
        }
        if !branch_succeeded {
            pipe.fetching = cpu.fetch(cpu.pc() as usize);
            cpu.increment_pc();
        }
        start_pipeline_helper(cpu, pipe);
    } else {
        let ended = end_pipeline(cpu, pipe);
        if !ended {
            start_pipeline_helper(cpu, pipe);
        }
    }
}


/// Function that tries to end the pipeline and returns whether it did actually
/// succeed in ending it
fn end_pipeline(cpu: &mut CpuState, pipe: &mut Pipe) -> bool {
    if let Some(instr) = &pipe.executing {
        let instr_type = instr.instruction_type;
        let succeeded = execute_instr(&Rc::clone(instr), cpu, pipe);
        if succeeded && (instr_type == InstructionType::BRANCH) {
            // executed a branch instruction which succeeded, so no longer terminating
            return false;
        }
        cpu.increment_pc();
        pipe.clear_decoding();
    } else {
        if let Some(instr) = &pipe.decoding {
            let instr_type = instr.instruction_type;
            let succeeded = execute_instr(&Rc::clone(instr), cpu, pipe);
            if succeeded && (instr_type == InstructionType::BRANCH) {
                // executed a branch instruction which succeeded, so no longer terminating
                return false;
            }
        }
        pipe.clear_decoding();
    }

    cpu.increment_pc();
    true
}
