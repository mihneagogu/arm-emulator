/// Macro that panics if the condition is true, with the given message
macro_rules! panic_msg {
    ($cond:expr, $msg:tt) => {
        if $cond {
            panic!($msg);
        }
    }
}

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
const MAX_BIT_INDEX: u8 = 31;

/// Enum that holds a position of a bit from a 32-bit number
pub enum BitPos32 {
    Pos(u8),
}

impl BitPos32 {

    /// Generates a Position from a u8
    ///
    /// # Panics
    /// Panics if given a position greater than 31
    pub fn from_u8(pos: u8) -> Self {
        if pos > MAX_BIT_INDEX {
            panic!(
                "You want a bit position from a 32-bit number \
            but gave me a number which is greater than 31"
            );
        }
        BitPos32::Pos(pos)
    }

    /// Gets the position from inside the wrapper as an u8
    pub fn to_u8(&self) -> u8 {
        match self {
            BitPos32::Pos(p) => *p
        }
    }

}

#[inline]
pub fn process_mask(n: u32, start_pos: BitPos32, end_pos: BitPos32) -> u32 {
    let end_pos = end_pos.to_u8();
    let start_pos = start_pos.to_u8();
    let mask: u32 = 1 << (end_pos + 1 - start_pos) - 1;
    (n >> start_pos) & mask
}

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

    /// Increments the ProgramCounter (registers[15])
    /// by 4 bytes aka 32 bits, passing to the next instruction
    pub fn increment_pc(&mut self) {
        self.registers[PC] += 4;
    }

    /// Offsets the ProgramCounter with 'offset' bytes
    /// It is guaranteed not to overflow u32 type so casting to i32 then subtracting
    /// and then casting back is fine
    pub fn offset_pc(&mut self, offset: i32) {
        self.registers[PC] += ((self.registers[PC] as i32) + offset) as u32;
    }

    /// Pretty prints the registers
    pub fn print_registers(&self) {
        let registers = &*self.registers;

        println!("Registers:");
        for (ind, reg) in registers.iter().enumerate() {
            let identifier = match ind {
                // Unused registers
                13 | 14 => continue,
                PC => {
                    String::from("$PC:   ")
                    //println!("$PC:    (0x{:0>8x})", reg);
                }
                CPSR => {
                    String::from("$CPSR: ")
                    //println!("$CPSR:  (0x{:0>8x})", reg);
                }
                n if n < 10 => {
                    format!("${}     ", ind)
                    //println!("${}:     (0x{:0>8x})", ind, reg);
                }
                _ => {
                    // n > 10
                    format!("${}    ", ind)
                    //println!("${}:    (0x{:0>8x})", ind, reg);
                }
            };
            println!("{}(0x{:0>8x})", identifier, reg);
        }
    }
}
