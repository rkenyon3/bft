//! Program to take a Brainfuck program at the specified file path and 'run' it

mod cli;

use std::{num::NonZeroUsize, path::PathBuf, process::ExitCode};

use bft_interp::VirtualMachine;
use bft_types::BfProgram;
use clap::Parser;

/// Struct to hold the parameters for running the bft.
struct BftParams {
    program_file: PathBuf,
    tape_cell_count: Option<NonZeroUsize>,
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

fn run_bft(params: BftParams) -> Result<(), Box<dyn std::error::Error>> {
    let mut bf_program = BfProgram::from_file(params.program_file)?;

    bf_program.analyse_program()?;

    let _bf_interpreter: VirtualMachine<u8> = VirtualMachine::new(
        bf_program,
        params.tape_cell_count,
        params.tape_is_extensible,
    );


    Ok(())
}

fn main() -> std::process::ExitCode {
    let args = cli::Args::parse(); // TODO: this should really be seperated out better

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
