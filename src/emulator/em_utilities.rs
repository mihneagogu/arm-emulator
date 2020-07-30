use std::fs;
use std::rc::Rc;

use num_derive::FromPrimitive;

/// Println!'s a statement
/// with the given format if the program is run in debug mode
macro_rules! debug_println {
    ($($args:tt)*) => {
        if cfg!(debug_assertions) {
            println!($($args)*);
        }
    };
}

#[doc = "Macro used for masking, just a shorthand"]
macro_rules! mask {
    ($bits:expr, $pos:expr) => {
        bit_mask($bits, bp32![$pos]) != 0
    };

    ($bits:expr, $start:expr, $end:expr) => {
        process_mask($bits, bp32![$start], bp32![$end])
    };
}

#[doc = "Macro that panics if the condition is true, with the given message"]
macro_rules! panic_on {
    ($cond:expr, $msg:tt) => {
        if $cond {
            panic!($msg);
        }
    };
}

#[doc = "Creates a BitPos32 wrapper from the given integer"]
macro_rules! bp32 {
    ($pos:expr) => {{
        BitPos32::from_u8($pos)
    }};
}

/// The state flags of the ARM processor
#[derive(Debug, FromPrimitive)]
pub enum Flag {
    N = 0,
    Z = 1,
    C = 2,
    V = 3,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Hash, PartialEq, Clone, Copy)]
pub enum InstructionType {
    DATA_PROCESS,
    MULTIPLTY,
    SINGLE_DATA_TRANSFER,
    BRANCH,
}

impl Eq for InstructionType {}

#[derive(Debug)]
pub struct Instruction {
    pub code: u32,
    pub instruction_type: InstructionType,
}

/// The byte code of the emulator conditions
#[derive(FromPrimitive, Debug)]
pub enum FlagCode {
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
            BitPos32::Pos(p) => *p,
        }
    }
}

/// Gives back the bits from start_pos until end_pos (both inclusive)
/// Shifted to the right, so it's readable
#[inline]
pub fn process_mask(n: u32, start_pos: BitPos32, end_pos: BitPos32) -> u32 {
    let end_pos = end_pos.to_u8();
    let start_pos = start_pos.to_u8();
    let mask: u32 = ( 1 << (end_pos + 1 - start_pos) ) - 1;
    (n >> start_pos) & mask
}

/// Checks if the given bit is set
pub fn bit_mask(n: u32, pos: BitPos32) -> u32 {
    (n >> pos.to_u8()) & 1
}

/// The pipeline struct
#[derive(Debug)]
pub struct Pipe {
    pub executing: Option<Rc<Instruction>>,
    pub decoding: Option<Rc<Instruction>>,
    pub fetching: u32,
}

impl Pipe {
    /// The pipeline lag is 8 bytes (aka 2 instructions)
    /// because of the pipeline execution cycle
    const PIPE_LAG: u8 = 8;
    pub fn init(cpu: &mut CpuState) -> Self {
        cpu.increment_pc();
        Self {
            executing: None,
            decoding: None,
            fetching: cpu.fetch(0),
        }
    }

    pub fn clear_executing(&mut self) {
        self.executing = None;
    }

    pub fn clear_decoding(&mut self) {
        self.decoding = None;
    }

    pub fn clear_fetching(&mut self) {
        self.fetching = 0;
    }

    pub fn set_fetching(&mut self, code: u32) {
        self.fetching = code;
    }

    pub fn clear(&mut self) {
        self.executing = None;
        self.decoding = None;
        self.fetching = 0;
    }
}

#[derive(Debug, PartialEq)]
pub struct CpuState {
    pub registers: Box<[u32]>,
    pub memory: Box<[u8]>,
}

impl Eq for CpuState {}

impl CpuState {
    /// Initializes an ARM Cpu
    /// with 17 registers
    /// and 65536 bytes of memory
    ///
    /// # Panics
    /// Panics if the number of bytes from the binary file isn't divisible by 4
    /// (Must mean the file is corrupted)
    pub fn init(path: &str) -> Result<Self, std::io::Error> {
        let mut instruction_vec = fs::read(path)?;
        panic_on!(
            instruction_vec.len() % 4 != 0,
            "Can only have a number of bytes in the file which is divisible by 4"
        );
        instruction_vec.resize(MEMORY_SIZE, 0);
        let memory: Box<[u8]> = instruction_vec.into_boxed_slice();
        Ok(Self {
            registers: Box::new([0; REGISTERS_NO]),
            memory,
        })
    }

    /// Fetches a big endian u32 at location ptr from the memory
    pub fn fetch_big_endian(&self, ptr: usize) -> u32 {
        self.index_big_endian(ptr)
    }

    /// Fetches a little endian u32 at location ptr from the memory
    pub fn fetch(&self, ptr: usize) -> u32 {
        self.index_little_endian(ptr)
    }

    /// Indexes in little endian an instruction from memory
    fn index_little_endian(&self, ptr: usize) -> u32 {
        self.memory[ptr] as u32
            | (self.memory[ptr + 1] as u32) << 8
            | (self.memory[ptr + 2] as u32) << 16
            | (self.memory[ptr + 3] as u32) << 24
    }

    /// Indexes in big endian an instruction from memory
    fn index_big_endian(&self, ptr: usize) -> u32 {
        (self.memory[ptr] as u32) << 24
            | (self.memory[ptr + 1] as u32) << 16
            | (self.memory[ptr + 2] as u32) << 8
            | (self.memory[ptr + 3] as u32)
    }

    /// Returns the current ProgramCounter value
    pub fn pc(&self) -> u32 {
        self.registers[PC]
    }

    pub fn cpsr(&self) -> u32 {
        self.registers[CPSR]
    }

    /// Sets the given CPSR flag
    #[allow(non_snake_case)]
    pub fn set_CPSR_flag(&mut self, flag: Flag, set: bool) {
        let mask: u32 = 1 << (31 - (flag as u32));
        self.registers[CPSR] = if set {
            self.cpsr() | mask
        } else {
            self.cpsr() & !mask
        };
    }

    /// Gets the CPSR status for the given flag
    pub fn get_flag(&self, flag: Flag) -> bool {
        let mask: u32 = 1 << (MAX_BIT_INDEX - flag as u8);
        // Parantheses probably not needed, added for good measure
        (self.registers[CPSR] & mask) != 0
    }

    /// Checks if the CPSR condition meets the flag requirements
    #[allow(non_snake_case)]
    pub fn check_CPSR_cond(&self, flag_code: FlagCode) -> bool {
        match flag_code {
            // Equal
            FlagCode::EQ => self.get_flag(Flag::Z),
            // Not equal
            FlagCode::NE => !self.get_flag(Flag::Z),
            // Greater than or equal
            FlagCode::GE => self.get_flag(Flag::N) == self.get_flag(Flag::V),
            // Less than
            FlagCode::LT => self.get_flag(Flag::N) != self.get_flag(Flag::V),
            // Greater than
            FlagCode::GT => {
                !self.get_flag(Flag::Z) && (self.get_flag(Flag::N) == self.get_flag(Flag::V))
            }
            // Less than or equal
            FlagCode::LE => {
                self.get_flag(Flag::Z) || (self.get_flag(Flag::N) != self.get_flag(Flag::V))
            }
            // Always true
            FlagCode::AL => true,
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
        self.registers[PC] = ((self.registers[PC] as i32) + offset) as u32;
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
            println!("{} {:>12} (0x{:0>8x})", identifier, reg, reg);
        }
    }
}
