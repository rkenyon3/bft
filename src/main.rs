//! Brainfuck Interpreter. This takes created a [BfProgram] from a file and runs it on a
//! virtual machine. The program is first analysed to confirm that the jump commands ('[' and ']')
//! are balanced.
//!
//! The virtual machine contains a tape of cells that can be under a read/write head. The size of
//! this tape may be specified as --cells cell_count, or will default to 30,000.
//!
//! The virtual machine iNput and output may be from stdin and stdout, or be specified as files
//! using --input file_name and --output file_name

mod cli;

use std::process::ExitCode;

use bft_interp::VirtualMachine;
use bft_types::BfProgram;
use clap::Parser;

use cli::Args;

/// Analyse the program for validity, then construct a [VirtualMachine] and
/// run it
///```no_run
/// let args = cli::Args::parse();
///
/// run_bft(&args)?;
///```
fn run_bft(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut bf_program = BfProgram::from_file(&args.program)?;

    let _bf_interpreter: VirtualMachine<u8> =
        VirtualMachine::new(&mut bf_program, args.cells, args.extensible)?;

    Ok(())
}

/// Main function
fn main() -> std::process::ExitCode {
    let args = cli::Args::parse();

    let run_result = run_bft(&args);
    match run_result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            println!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
