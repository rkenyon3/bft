//! Brainfuck Interpreter. This crate takes created a [BfProgram] from a file and runs it on a
//! virtual machine. The program is first analysed to confirm that the jump commands ('[' and ']')
//! are balanced.
//!
//! The virtual machine contains a tape of cells that can be under a read/write head. The size of
//! this tape may be specified as --cells cell_count, or will default to 30,000.
//!
//! The virtual machine iNput and output may be from stdin and stdout, or be specified as files
//! using --input file_name and --output file_name

mod cli;

use std::{num::NonZeroUsize, path::PathBuf, process::ExitCode};

use bft_interp::VirtualMachine;
use bft_types::BfProgram;
use clap::Parser;

/// Parameters for creation of a [VirtualMachine]
struct BftParams {
    /// Path of the program file containing a bf program
    program_file: PathBuf,
    /// Number of cells on the VM tape. May be None for default size
    tape_cell_count: Option<NonZeroUsize>,
    /// Determines whether the tape may automatically when the head reaches the
    /// end
    tape_is_extensible: bool,
}

impl BftParams {
    fn new(
        program_file: PathBuf,
        tape_cell_count: Option<NonZeroUsize>,
        tape_is_extensible: bool,
    ) -> Self {
        Self {
            program_file,
            tape_cell_count,
            tape_is_extensible,
        }
    }
}

/// Analyse the program for validity, then construct a [VirtualMachine] and
/// run it
///```no_run
/// let args = cli::Args::parse();
/// let params = BftParams::new(args.program, args.cells, args.extensible);
///
/// run_bft(params)?;
///```
fn run_bft(params: BftParams) -> Result<(), Box<dyn std::error::Error>> {
    let mut bf_program = BfProgram::from_file(params.program_file)?;

    let _bf_interpreter: VirtualMachine<u8> = VirtualMachine::new(
        &mut bf_program,
        params.tape_cell_count,
        params.tape_is_extensible,
    )?;

    Ok(())
}

/// Main function
fn main() -> std::process::ExitCode {
    let args = cli::Args::parse();

    let params = BftParams::new(args.program, args.cells, args.extensible);

    let run_result = run_bft(params);
    match run_result {
        Ok(()) => {
            println!("Done");
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
