//! CLI interface arguments for the Brainfuck interpreter

use std::num::NonZeroUsize;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Path to the file containing the brainfuck program. Required.
    pub program: std::path::PathBuf,

    /// Initial size of the VM's tape.
    #[arg(short, long)]
    pub cells: Option<NonZeroUsize>,

    /// whether the tape can grow automatically
    #[arg(short, long)]
    pub extensible: bool,
}
