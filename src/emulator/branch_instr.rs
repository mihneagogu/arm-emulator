
use num_traits::FromPrimitive;

use crate::emulator::em_utilities as util;
use util::*;

/// Executes a branch instruction, returning whether it succeeded or not
pub fn execute_branch_instr(instr: &Instruction, cpu: &mut CpuState, pipe: &mut Pipe) -> bool {
    let condition = process_mask(instr.code, BitPos32::from_u8(28), BitPos32::from_u8(31));
    let condition = FromPrimitive::from_u32(condition).unwrap();

    if !cpu.check_CPSR_cond(condition) {
        // Failed jump condition, not jumping, clearing the current branch instruction
        // from the pipe
        pipe.clear_executing();
        return false;
    }

    // Processes offset from bits 0-23
    let mut offset: i32 =
        (process_mask(instr.code, BitPos32::from_u8(0), BitPos32::from_u8(23)) << 2) as i32;

    if bit_mask(offset as u32, BitPos32::from_u8(25)) != 0 {
        // Must be a negative number, sign extending it
        let mask: i32 = 1 << 26;
        offset |= -mask;
    }

    cpu.offset_pc(offset);
    pipe.clear();
    pipe.set_fetching(cpu.fetch(cpu.pc() as usize));
    cpu.increment_pc();

    true
}
