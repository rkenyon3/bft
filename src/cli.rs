//! CLI arguments for the Brainfuck interpreter

use std::num::NonZeroUsize;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Path to the file containing the brainfuck program. Required.
    pub program: PathBuf,

    /// Initial size of the VM's tape.
    #[arg(short, long)]
    pub cells: Option<NonZeroUsize>,

    /// Controls whether the end of tape will be extended automatically
    #[arg(short, long)]
    pub extensible: bool,
}
