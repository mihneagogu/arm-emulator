/// The state flags of the ARM processor
#[derive(Debug)]
enum Flag {
    N = 0,
    Z = 1,
    C = 2,
    V = 3,
}

/// The byte code of the emulator conditions
#[derive(Debug)]
enum FlagCodes {
    EQ = 0,
    NE = 1,
    GE = 10,
    LT = 11,
    GT = 12,
    LE = 13,
    AL = 14,
}

const REGISTERS_NO: usize = 17;
const MEMORY_SIZE: usize = 65536;
const PC: usize = 15;
const CPSR: usize = 16;

#[derive(Debug)]
pub struct CpuState {
    registers: Box<[u32]>,
    memory: Box<[u8]>,
}

impl CpuState {
    /// Initializes an ARM Cpu
    /// with 17 registers
    /// and 65536 bytes of memory
    pub fn init() -> Self {
        Self {
            registers: Box::new([0; REGISTERS_NO]),
            memory: Box::new([0; MEMORY_SIZE]),
        }
    }

    /// Pretty prints the registers 
    pub fn print_registers(&self) {
        let registers = &*self.registers;

        println!("Registers:");
        for (ind, reg) in registers.iter().enumerate() {
            match ind {
                // Unused registers
                13 | 14 => continue,
                PC => {
                    println!("$PC:    (0x{:0>8x})", reg);
                }
                CPSR => {
                    println!("$CPSR:  (0x{:0>8x})", reg);
                }
                n if n < 10 => {
                    println!("${}:     (0x{:0>8x})", ind, reg);
                }
                _ => {
                    println!("${}:    (0x{:0>8x})", ind, reg);

                }
            };
        }
    }
}
