#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn add01() {
        let cpu = emulate("tests/add01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let mut expected_reg: Vec<u32> = vec![0; 17];
        let mut expected_mem: Vec<u8> = vec![0; 65536];

        expected_reg[1] = 1;
        expected_reg[2] = 3;
        expected_reg[15] = 16;

        let mut expected = CpuState {
            registers: expected_reg.into_boxed_slice(),
            memory : expected_mem.into_boxed_slice()
        };
        registers_eq(&mut cpu, &mut expected);
    }
}
