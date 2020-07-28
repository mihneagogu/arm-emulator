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
    pipeline_executor::emulate(path)?;

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
