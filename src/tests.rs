/// It is expected that the loading of the binary file is done properly in memory
#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Check memory as well

    const PC: usize = 15;
    const CPSR: usize = 16;

    use crate::emulator::em_utilities as util;
    use crate::emulator::pipeline_executor::emulate;
    use util::*;

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
        let mut expected_reg: Vec<u32> = vec![0; 17];
        let expected_mem: Vec<u8> = vec![0; 65536];

        expected_reg[1] = 1;
        expected_reg[2] = 3;
        expected_reg[PC] = 16;
        expected_reg[CPSR] = 0;

        let mut expected = CpuState {
            registers: expected_reg.into_boxed_slice(),
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
        let mut expected_reg: Vec<u32> = vec![0; 17];
        let expected_mem: Vec<u8> = vec![0; 65536];

        expected_reg[1] = 1;
        expected_reg[2] = 2;
        expected_reg[3] = 3;
        expected_reg[PC] = 20;
        expected_reg[CPSR] = 0;

        let mut expected = CpuState {
            registers: expected_reg.into_boxed_slice(),
            memory : expected_mem.into_boxed_slice()
        };
        registers_eq(&mut cpu, &mut expected);
    }


    #[test]
    fn add03() {
        let cpu = emulate("tests/add03");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let mut expected_reg: Vec<u32> = vec![0; 17];
        let expected_mem: Vec<u8> = vec![0; 65536];

        expected_reg[1] = 2;
        expected_reg[PC] = 16;
        expected_reg[CPSR] = 0;

        let mut expected = CpuState {
            registers: expected_reg.into_boxed_slice(),
            memory : expected_mem.into_boxed_slice()
        };
        registers_eq(&mut cpu, &mut expected);
    }

    #[test]
    fn add04() {
        let cpu = emulate("tests/add04");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let mut expected_reg: Vec<u32> = vec![0; 17];
        let expected_mem: Vec<u8> = vec![0; 65536];
        
        expected_reg[1] = 1;
        expected_reg[2] = 2;
        expected_reg[3] = 3;
        expected_reg[4] = 7;
        expected_reg[PC] = 24;
        expected_reg[CPSR] = 0;


        let mut expected = CpuState {
            registers: expected_reg.into_boxed_slice(),
            memory : expected_mem.into_boxed_slice()
        };
        registers_eq(&mut cpu, &mut expected);
    }

    #[test]
    fn and01() {
        let cpu = emulate("tests/and01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let mut expected_reg: Vec<u32> = vec![0; 17];
        let expected_mem: Vec<u8> = vec![0; 65536];
        
        expected_reg[1] = 255;
        expected_reg[2] = 171;
        expected_reg[PC] = 16;
        expected_reg[CPSR] = 0;


        let mut expected = CpuState {
            registers: expected_reg.into_boxed_slice(),
            memory : expected_mem.into_boxed_slice()
        };
        registers_eq(&mut cpu, &mut expected);
    }

    #[test]
    fn and03() {
        let cpu = emulate("tests/and02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let mut expected_reg: Vec<u32> = vec![0; 17];
        let expected_mem: Vec<u8> = vec![0; 65536];
        
        expected_reg[1] = 15;
        expected_reg[2] = 171;
        expected_reg[3] = 11;
        expected_reg[PC] = 20;
        expected_reg[CPSR] = 0;


        let mut expected = CpuState {
            registers: expected_reg.into_boxed_slice(),
            memory : expected_mem.into_boxed_slice()
        };
        registers_eq(&mut cpu, &mut expected);
    }

}
