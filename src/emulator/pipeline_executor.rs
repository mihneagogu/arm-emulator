mod em_utilities;
use em_utilities::CpuState;

/// Executes the emulator given the instruction vector
pub fn emulate(instructions: Vec<u32>) {
    let cpu = CpuState::init();
    cpu.print_registers();
}
