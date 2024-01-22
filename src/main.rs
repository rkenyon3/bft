//! Program to take a Brainfuck program at the specified file path and 'run' it

use bft_interp::VirtualMachine;
use bft_types::BfProgram;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = PathBuf::from(env::args_os().nth(1).ok_or("Needs BF program file name")?);

    let bf_program = BfProgram::from_file(file_path)?;
    let bf_interpreter: VirtualMachine<u8> = VirtualMachine::new(None, false);

    bf_interpreter.print_program(&bf_program);

    Ok(())
}
