use std::process::exit;
use std::{env, fs};

mod emulator;
use emulator::pipeline_executor;



/// Println!'s a statement
/// with the given format if the program is run in debug mode
macro_rules! debug_println {
    ($($args:tt)*) => {
        if cfg!(debug_assertions) {
            println!($($args)*);
        }
    };
}
#[derive(Debug)]
enum Task<'a> {
    Emulate(&'a str),
    Assemble {
        asm_path: &'a str,
        out_path: &'a str,
    },
}

/// Runs the emulator or assembler
/// Run it using this command:
/// emulate <binary-file-path>
/// assemble <asm-file-path> <output-path>
///
/// # Panics
///
/// Panics if run with wrong command-line parameters
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let task_description = assert_cmd_line_params(&args);

    match task_description {
        Task::Emulate(path) => emulate(path),
        Task::Assemble { asm_path, out_path } => assemble(asm_path, out_path),
    }
}

fn assemble(asm_path: &str, out_path: &str) -> Result<(), std::io::Error> {
    Ok(())
}

/// Reads an emulator binary file which contains lines of u32 
/// and produces the required output
///
/// # Panics
/// Panics if the file has a number of bytes indivisible by 4
///
/// Propagates std::io::Error to `main` if the file path is invalid
fn emulate(path: &str) -> Result<(), std::io::Error> {
    let instructions = std::fs::read(path)?;

    if instructions.len() % 4 != 0 {
        panic!("Binary file has a number of bytes undivisible by 4");
    }

    let mut counter = 0u8;

    // array of u32 for convenience, so we do not need to convert
    // every time from u8 to u32 when shifting
    let mut instr_bytes: [u32; 4] = [0; 4];
    let mut u32_instructions: Vec<u32> = Vec::with_capacity(instructions.len() % 4);
    let mut first = true;

    // Parses the instruction from the binary file
    for b in &instructions {
        if counter % 4 == 0 && !first {
            // Index it in little endian
            let instr: u32 =
                instr_bytes[0] | instr_bytes[1] << 8 | instr_bytes[2] << 16 | instr_bytes[3] << 24;
            u32_instructions.push(instr);
            counter = 0;
        }
        first = false;
        instr_bytes[counter as usize] = *b as u32;
        counter += 1;
    }

    // Just shadowing the `instruction` variable
    let instructions = u32_instructions;

    if cfg!(debug_assertions) {
        print!("Binary instructions as little endian u32: ");
        print!("[");
        for inst in &instructions {
            print!("{:x}, ", inst);
        }
        println!("]");
    }

    // Starts the emulation process
    pipeline_executor::emulate(instructions);

    Ok(())
}

fn assert_cmd_line_params(args: &[String]) -> Task {
    let good_len = args.len() == 3 || args.len() == 4;
    if !good_len {
        panic!("You gave me a wrong command format, please check the documentation!");
    }

    let TASK_INDEX: usize= 1;
    let FILE_PATH_INDEX: usize = 2;
    let OUT_PATH_INDEX: usize = 3;

    if &args[TASK_INDEX] == "emulate" {
        return Task::Emulate(&args[FILE_PATH_INDEX]);
    }
    if &args[TASK_INDEX] == "assemble" {
        if args.len() != 4 {
            panic!("Wrong assemble information! Please use `assemble <asm-path> <output-path>`");
        }
        return Task::Assemble {
            asm_path: &args[FILE_PATH_INDEX],
            out_path: &args[OUT_PATH_INDEX],
        };
    }
    panic!("The first argument must be either `emulate` or `assemble`");
}
