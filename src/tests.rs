#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Add all tests!

    const PC: usize = 15;
    const CPSR: usize = 16;

    use crate::emulator::em_utilities as util;
    use crate::emulator::pipeline_executor::emulate;
    use util::*;

    #[doc = "empty vector for memory, just for creating a CpuState"]
    macro_rules! mem_empty {
        () => {
            vec![].into_boxed_slice()
        };
    }

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
            let got = cpu.fetch_big_endian(*ptr);
            assert!(
                got == *expected,
                "Mismatch on memory at 0x{:x}, expected: 0x{:x}, found: 0x{:x}",
                ptr,
                expected,
                got
            );
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
            memory: expected_mem.into_boxed_slice(),
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(&mut cpu, vec![(0, 0x0110a0e3), (4, 0x022081e2)]);
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
            memory: expected_mem.into_boxed_slice(),
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![(0, 0x0110a0e3), (4, 0x0220a0e3), (8, 0x023081e0)],
        );
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
            memory: expected_mem.into_boxed_slice(),
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(&mut cpu, vec![(0, 0x0110a0e3), (4, 0x011081e0)]);
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
            memory: expected_mem.into_boxed_slice(),
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3),
                (4, 0x0220a0e3),
                (8, 0x023081e0),
                (0xc, 0x044083e2),
            ],
        );
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
            memory: expected_mem.into_boxed_slice(),
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(&mut cpu, vec![(0, 0xff10a0e3), (4, 0xab2001e2)]);
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
            memory: expected_mem.into_boxed_slice(),
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![(0, 0x0f10a0e3), (4, 0xab20a0e3), (8, 0x023001e0)],
        );
    }

    #[test]
    fn b01() {
        let cpu = emulate("tests/b01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 1), (3, 3), (PC, 24)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: vec![].into_boxed_slice(),
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3),
                (4, 0x000000ea),
                (8, 0x0220a0e3),
                (0xc, 0x0330a0e3),
            ],
        );
    }

    #[test]
    fn beq01() {
        let cpu = emulate("tests/beq01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 1), (2, 1), (4, 4), (PC, 32), (CPSR, 0x60000000)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3),
                (4, 0x0120a0e3),
                (8, 0x020051e1),
                (0xc, 0x0000000a),
                (0x10, 0x0330a0e3),
                (0x14, 0x0440a0e3),
            ],
        );
    }

    #[test]
    fn beq02() {
        let cpu = emulate("tests/beq02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 1), (2, 2), (3, 3), (4, 4), (PC, 32), (CPSR, 0x80000000)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3),
                (4, 0x0220a0e3),
                (8, 0x020051e1),
                (0xc, 0x0000000a),
                (0x10, 0x0330a0e3),
                (0x14, 0x0440a0e3),
            ],
        );
    }

    #[test]
    fn bne01() {
        let cpu = emulate("tests/bne01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 1), (2, 1), (3, 3), (4, 4), (PC, 32), (CPSR, 0x60000000)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3),
                (4, 0x0120a0e3),
                (8, 0x020051e1),
                (0xc, 0x0000001a),
                (0x10, 0x0330a0e3),
                (0x14, 0x0440a0e3),
            ],
        );
    }

    #[test]
    fn bne02() {
        let cpu = emulate("tests/bne02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 1), (2, 2), (4, 4), (PC, 32), (CPSR, 0x80000000)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3),
                (4, 0x0220a0e3),
                (8, 0x020051e1),
                (0xc, 0x0000001a),
                (0x10, 0x0330a0e3),
                (0x14, 0x0440a0e3),
            ],
        );
    }

    #[test]
    fn eor01() {
        let cpu = emulate("tests/eor01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 15), (2, 0xff), (3, 0xf0), (PC, 20)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![(0, 0x0f10a0e3), (4, 0xff20a0e3), (8, 0x023021e0)],
        );
    }

    #[test]
    fn eor02() {
        let cpu = emulate("tests/eor02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 0xff), (2, 0xf0), (PC, 16)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(&mut cpu, vec![(0, 0x0ff10a0e3), (4, 0x0f2021e2)]);
    }

    #[test]
    fn factorial() {
        let cpu = emulate("tests/factorial");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![
            (0, 120),
            (2, 120),
            (3, 0x100),
            (PC, 0x2c),
            (CPSR, 0x60000000),
        ];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0100a0e3),
                (4, 0x0510a0e3),
                (8, 0x910002e0),
                (0xc, 0x0200a0e1),
                (0x10, 0x011041e2),
                (0x14, 0x000051e3),
                (0x18, 0xfaffff1a),
                (0x1c, 0x013ca0e3),
                (0x20, 0x002083e5),
                (0x100, 0x78000000),
            ],
        );
    }

    #[test]
    fn ldr01() {
        let cpu = emulate("tests/ldr01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(0, 2), (2, 0x2000e3a0), (PC, 20), (CPSR, 0x20000000)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0200a0e3),
                (4, 0x002090e5),
                (8, 0x000052e1)
            ],
        );
    }

    #[test]
    fn ldr02() {
        let cpu = emulate("tests/ldr02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(0, 0x03000000), (2, 225), (PC, 24), (CPSR, 0x80000000)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0f00a0e3),
                (4, 0x002090e5),
                (8, 0x0304a0e3),
                (0xc, 0x000052e1)

            ],
        );
    }

    #[test]
    fn ldr03() {
        let cpu = emulate("tests/ldr03");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(0, 11), (1, 0xfffffffe), (2, 0x411005e3), (3, 0x28000), (PC, 0x1c)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0b00a0e3),
                (4, 0x002090e5),
                (8, 0x0310a0e3),
                (0xc, 0x051041e2),
                (0x10, 0x0a39a0e3)
            ],
        );
    }

    #[test]
    fn ldr07() {
        let cpu = emulate("tests/ldr07");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(2, 0x20200020), (PC, 12)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x00209fe5),
                (8, 0x20002020)
            ],
        );
    }

    #[test]
    fn ldr08() {
        let cpu = emulate("tests/ldr08");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(0, 0x20200020), (2, 0x20200030), (PC, 0x14)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x08009fe5),
                (4, 0x08209fe5), 
                (8, 0x082082e2),
                (0x10, 0x20002020),
                (0x14, 0x28002020)
            ],
        );
    }

    #[test]
    fn ldr09() {
        let cpu = emulate("tests/ldr09");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 0x20200022), (2, 0x20200020), (PC, 0x14)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x08209fe5),
                (4, 0x021082e2),
                (8, 0x020033e3),
                (0x10, 0x20002020)
            ],
        );
    }

    #[test]
    fn ldr14() {
        let cpu = emulate("tests/ldr14");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(0, 0x20200020), (2, 0x20200030), (PC, 0x14)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x08009fe5),
                (4, 0x08209fe5),
                (8, 0x082082e2),
                (0x10, 0x20002020),
                (0x14, 0x28002020)
            ],
        );
    }

    #[test]
    fn ldr15() {
        let cpu = emulate("tests/ldr15");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 4), (3, 0xe5913004), (PC, 0x14)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3),
                (4, 0x031081e2),
                (8, 0x043091e5)
            ],
        );
    }

    #[test]
    fn ldr16() {
        let cpu = emulate("tests/ldr16");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 8), (3, 0xe2811003), (PC, 0x14)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0510a0e3),
                (4, 0x031081e2),
                (8, 0x043011e5)
            ],
        );
    }

    #[test]
    fn loop01() {
        let cpu = emulate("tests/loop01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(2, 0xff), (PC, 24), (CPSR, 0x60000000)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x3f28a0e3),
                (4, 0x012042e2),
                (8, 0xff0052e3),
                (0xc, 0xfcffff1a)
            ],
        );
    }

    #[test]
    fn loop02() {
        let cpu = emulate("tests/loop02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(PC, 40), (CPSR, 0x60000000)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0520a0e3),
                (4, 0x012042e2),
                (8, 0x0a10a0e3),
                (0xc, 0x011041e2),
                (0x10, 0x020051e1),
                (0x14, 0xfcffff1a),
                (0x18, 0x000052e3),
                (0x1c, 0xf8ffff1a)
            ],
        );
    }

    #[test]
    fn loop03() {
        let cpu = emulate("tests/loop03");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(PC, 24), (CPSR, 0x60000000)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0a20a0e3),
                (4, 0x012042e2),
                (8, 0x000052e3),
                (0xc, 0xfcffff1a)
            ],
        );
    }

    #[test]
    fn lsl01() {
        let cpu = emulate("tests/lsl01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 2), (PC, 16)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3),
                (4, 0x8110a0e1)
            ],
        );
    }

    #[test]
    fn lsl02() {
        let cpu = emulate("tests/lsl02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 0x80000000), (PC, 16)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0xff10a0e3),
                (4, 0x811fa0e1)
            ],
        );
    }

    #[test]
    fn mla01() {
        let cpu = emulate("tests/mla01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 2), (2, 4), (3, 12), (4, 4), (PC, 24)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0210a0e3),
                (4, 0x0420a0e3),
                (8, 0x0440a0e3),
                (0xc, 0x914223e0)
            ],
        );
    }

    #[test]
    fn mov01() {
        let cpu = emulate("tests/mov01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 1), (PC, 12)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3)
            ],
        );
    }

    #[test]
    fn mov02() {
        let cpu = emulate("tests/mov02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 2), (PC, 12)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0210a0e3)
            ],
        );
    }

    #[test]
    fn mov03() {
        let cpu = emulate("tests/mov03");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 15), (2, 0xff), (3, 171), (4, 0xcd), (PC, 24)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0f10a0e3),
                (4, 0xff20a0e3),
                (8, 0xab30a0e3),
                (0xc, 0xcd40a0e3)
            ],
        );
    }

    #[test]
    fn mov04() {
        let cpu = emulate("tests/mov04");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(2, 0x3f0000), (PC, 12)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x3f28a0e3)
            ],
        );
    }

    #[test]
    fn mov05() {
        let cpu = emulate("tests/mov05");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 1), (2, 1), (PC, 16)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3),
                (4, 0x0120a0e1)
            ],
        );
    }

    #[test]
    fn mov06() {
        let cpu = emulate("tests/mov06");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(2, 0x28000), (PC, 12)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0a29a0e3)
            ],
        );
    }

    #[test]
    fn mov07() {
        let cpu = emulate("tests/mov07");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(0, 0x03000000), (PC, 12)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0304a0e3)
            ],

        );
    }

    #[test]
    fn mul01() {
        let cpu = emulate("tests/mul01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 2), (2, 4), (3, 8), (PC, 20)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0210a0e3),
                (4, 0x0420a0e3),
                (8, 0x910203e0)
            ],

        );
    }

    #[test]
    fn opt_add05() {
        let cpu = emulate("tests/opt_add05");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 4), (2, 2), (3, 6), (4, 10), (5, 9), (PC, 28)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0410a0e3),
                (4, 0x0220a0e3),
                (8, 0x023081e0),
                (0xc, 0x44083e2),
                (0x10, 0x335244e0)
            ],

        );
    }

    #[test]
    fn opt_ldr10() {
        let cpu = emulate("tests/opt_ldr10");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 12), (2, 10), (3, 14), (4, 0x5122e593), (5, 0x5122e593), (PC, 28)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0c10a0e3),
                (4, 0x0a20a0e3),
                (8, 0x0e30a0e3),
                (0xc, 0x004093e5),
                (0x10, 0x225191e7)
            ],

        );
    }

    #[test]
    fn or01() {
        let cpu = emulate("tests/or01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(2, 171), (PC, 16)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0010a0e3),
                (4, 0xab2081e3)
            ],

        );
    }

    #[test]
    fn or02() {
        let cpu = emulate("tests/or02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 15), (2, 171), (3, 175), (PC, 20)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0f10a0e3),
                (4, 0xab20a0e3),
                (8, 0x023081e1)
            ],

        );
    }

    #[test]
    fn rsb01() {
        let cpu = emulate("tests/rsb01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 2), (2, 0xffffffff), (PC, 16)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0210a0e3),
                (4, 0x012061e2)
            ],

        );
    }

    #[test]
    fn rsb02() {
        let cpu = emulate("tests/rsb02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 0xff), (PC, 16)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0xff10a0e3),
                (4, 0xff2061e2)
            ],

        );
    }

    #[test]
    fn rsb03() {
        let cpu = emulate("tests/rsb03");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 0xff), (PC, 16)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0xff10a0e3),
                (4, 0x012061e0)
            ],

        );
    }

    #[test]
    fn str01() {
        let cpu = emulate("tests/str01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(0, 0x20200000), (PC, 16)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x04009fe5),
                (4, 0x1c0081e5),
                (0xc, 0x00002020),
                (0x1c, 0x00002020)
            ],

        );
    }

    #[test]
    fn str02() {
        let cpu = emulate("tests/str02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 2), (2, 5), (4, 1), (PC, 24)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0140a0e3),
                (4, 0x02000000),
                (8, 0x0210a0e3),
                (0xc, 0x041082e6)
            ],

        );
    }

    #[test]
    fn str03() {
        let cpu = emulate("tests/str03");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(0, 1), (1, 99), (PC, 40), (CPSR, 0x60000000)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0220a0e3),
                (4, 0x63000000),
                (8, 0x63630000),
                (0xc, 0x001080e5),
                (0x10, 0x012042e2),
                (0x14, 0x000052e3),
                (0x18, 0x081080e5),
                (0x1c, 0xfbffff1a)
            ],

        );
    }

    #[test]
    fn str04() {
        let cpu = emulate("tests/str04");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 35), (2, 4), (4, 1), (PC, 24)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0140a0e3),
                (4, 0x04230000),
                (8, 0x0010a0e3),
                (0xc, 0x041082e7)
            ],

        );
    }

    #[test]
    fn sub01() {
        let cpu = emulate("tests/sub01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 2), (2, 1), (PC, 16)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0210a0e3),
                (4, 0x012041e2)
            ],

        );
    }

    #[test]
    fn sub02() {
        let cpu = emulate("tests/sub02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 0xff), (PC, 16)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0xff10a0e3),
                (4, 0xff2041e2)
            ],

        );
    }

    #[test]
    fn sub03() {
        let cpu = emulate("tests/sub03");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 0xff), (PC, 16)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0xff10a0e3),
                (4, 0x012041e0)
            ],

        );
    }

    #[test]
    fn tst01() {
        let cpu = emulate("tests/tst01");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 1), (2, 2), (4, 4), (PC, 32), (CPSR, 0x40000000)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3),
                (4, 0x0220a0e3),
                (8, 0x020011e1),
                (0xc, 0x0000000a),
                (0x10, 0x0330a0e3),
                (0x14, 0x0440a0e3)
            ],

        );
    }

    #[test]
    fn tst02() {
        let cpu = emulate("tests/tst02");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 1), (2, 1), (3, 3), (4, 4), (PC, 32)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3),
                (4, 0x0120a0e3),
                (8, 0x020011e1),
                (0xc, 0x0000000a),
                (0x10, 0x0330a0e3),
                (0x14, 0x0440a0e3)
            ],

        );
    }

    #[test]
    fn tst03() {
        let cpu = emulate("tests/tst03");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 1), (2, 23), (4, 4), (PC, 32)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3),
                (4, 0x1720a0e3),
                (8, 0x020011e1),
                (0xc, 0x000000ea),
                (0x10, 0x0330a0e3),
                (0x14, 0x0440a0e3)
            ],

        );
    }

    #[test]
    fn tst04() {
        let cpu = emulate("tests/tst04");
        assert!(cpu.is_ok());
        let mut cpu = cpu.unwrap();
        let registers_special = vec![(1, 1), (2, 23), (4, 4), (PC, 32)];

        let mut expected = CpuState {
            registers: reg_from(registers_special),
            memory: mem_empty![],
        };
        registers_eq(&mut cpu, &mut expected);
        memory_eq(
            &mut cpu,
            vec![
                (0, 0x0110a0e3),
                (4, 0x1720a0e3),
                (8, 0x020011e1),
                (0xc, 0x000000ea),
                (0x10, 0x0330a0e3),
                (0x14, 0x0440a0e3)
            ],

        );
    }
}
