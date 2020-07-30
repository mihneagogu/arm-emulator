#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Add all tests!

    const PC: usize = 15;
    const CPSR: usize = 16;

    use crate::emulator::em_utilities as util;
    use crate::emulator::pipeline_executor::emulate;
    use util::*;

    /// Returns a heap-allocated [u32; 17] with all values equal to 0,
    /// except the special ones at `usize` index with `u32` value
    fn reg_from(specials: Vec<(usize, u32)>) -> Box<[u32]> {
        let mut registers: Vec<u32> = vec![0; 17];
        for (ind, val) in &specials {
            registers[*ind] = *val;
        }
        registers.into_boxed_slice()
    }

    /// Asserts whether the two cpus' registers are equal
    fn registers_eq(cpu: &mut CpuState, expected: &mut CpuState) {
        let cpu_reg = &*cpu.registers;
        let expected = &*expected.registers;
        assert!(
            (cpu_reg.len() == expected.len()) && cpu_reg.len() == 17,
            "Expect length of exactly 17 for the registers"
        );

        for (ind, &elem) in cpu_reg.iter().enumerate() {
            assert!(
                elem == expected[ind],
                "Register mismatch at index: {}, expected: {}, found: {}",
                ind,
                expected[ind],
                elem
            );
        }
    }

    /// Checks whether the memory is well laid-out
    /// Besides the given tuples of memory, everything else should be just 0s
    fn memory_eq(cpu: &mut CpuState, expected: Vec<(usize, u32)>) {
        for (ptr, expected) in &expected {
            assert!(cpu.fetch(*ptr) == *expected, "Mismatch on memory at 0x{:x}, expected: 0x{:x}, found: 0x{:x}", ptr, expected, cpu.fetch(*ptr));
        }

    }



    #[test]
    fn add01() {
        let cpu = emulate("tests/add01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let expected_mem: Vec<u8> = vec![0; 65536];
        let registers_special = vec![(1, 1), (2, 3), (PC, 16), (CPSR, 0)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory : expected_mem.into_boxed_slice()
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(&mut cpu, vec![(0, 0xe3a01001), (4, 0xe2812002)]);
    }

    #[test]
    fn add02() {
        let cpu = emulate("tests/add02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let expected_mem: Vec<u8> = vec![0; 65536];
        let registers_special = vec![(1, 1), (2, 2), (3, 3), (PC, 20), (CPSR, 0)]; 

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory : expected_mem.into_boxed_slice()
        };
        registers_eq(&mut cpu, &mut expected);
    }


    #[test]
    fn add03() {
        let cpu = emulate("tests/add03");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let expected_mem: Vec<u8> = vec![0; 65536];
        let special_registers = vec![(1, 2), (PC, 16), (CPSR, 0)];

        let mut expected = CpuState {
            registers: reg_from(special_registers),
            memory : expected_mem.into_boxed_slice()
        };
        registers_eq(&mut cpu, &mut expected);
    }

    #[test]
    fn add04() {
        let cpu = emulate("tests/add04");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let expected_mem: Vec<u8> = vec![0; 65536];
        let special_registers = vec![(1, 1), (2, 2), (3, 3), (4, 7), (PC, 24), (CPSR, 0)];
        
        let mut expected = CpuState {
            registers: reg_from(special_registers),
            memory : expected_mem.into_boxed_slice()
        };
        registers_eq(&mut cpu, &mut expected);
    }

    #[test]
    fn and01() {
        let cpu = emulate("tests/and01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let expected_mem: Vec<u8> = vec![0; 65536];
        let special_registers = vec![(1, 255), (2, 171), (PC, 16), (CPSR, 0)];
        
        let mut expected = CpuState {
            registers: reg_from(special_registers),
            memory : expected_mem.into_boxed_slice()
        };
        registers_eq(&mut cpu, &mut expected);
    }

    #[test]
    fn and02() {
        let cpu = emulate("tests/and02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let expected_mem: Vec<u8> = vec![0; 65536];
        let special_registers = vec![(1, 15), (2, 171), (3, 11), (PC, 20), (CPSR, 0)]; 

        let mut expected = CpuState {
            registers: reg_from(special_registers),
            memory : expected_mem.into_boxed_slice()
        };
        registers_eq(&mut cpu, &mut expected);
    }

}
