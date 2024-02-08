//! Program to take a Brainfuck program at the specified file path and 'run' it

mod cli;

use bft_interp::VirtualMachine;
use bft_types::BfProgram;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse(); // TODO: this should really be seperated out better

    let bf_program = BfProgram::from_file(args.program)?;

    bf_program.analyse_program()?;

    let bf_interpreter: VirtualMachine<u8> = VirtualMachine::new(args.cells, args.extensible);

    bf_interpreter.interpret_program(&bf_program);

    Ok(())
}
