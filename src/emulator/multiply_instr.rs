use crate::emulator::em_utilities::*;

macro_rules! accumulate_bits {
    ($bits:expr) => {
        mask![$bits, 21]
    }
}

macro_rules! reg_d_bits {
    ($bits:expr) => {
       mask![$bits, 16, 19] as usize
    }

}

macro_rules! reg_n_bits {
    ($bits:expr) => {
        mask![$bits, 12, 15] as usize
    }
}

macro_rules! reg_s_bits {
    ($bits:expr) => {
        mask![$bits, 8, 11] as usize
    }
}

macro_rules! reg_m_bits {
    ($bits:expr) => {
        mask![$bits, 0, 3] as usize
    }
}


pub fn execute_multiply_instruction(instr: &Instruction, cpu: &mut CpuState){
    let bits = instr.code;
    let set = mask![bits, 20];
    let mut result: u32 = cpu.registers[reg_m_bits![bits]] * cpu.registers[reg_s_bits![bits]];

    if accumulate_bits![bits] {
        result += cpu.registers[reg_n_bits![bits]];
    }
    
    if set {
        cpu.set_CPSR_flag(Flag::N, mask![result, 31]);
        if result == 0 {
            cpu.set_CPSR_flag(Flag::N, true);
        }
    }

    cpu.registers[reg_d_bits![bits]] = result;

}
