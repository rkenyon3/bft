use std::num::NonZeroUsize;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// name of the file containing the program
    pub program: std::path::PathBuf,

    // TODO: Improve parsing error message
    /// number of cells on the tape
    #[arg(short, long)]
    pub cells: Option<NonZeroUsize>,

    /// whether the tape can grow automatically
    #[arg(short, long)]
    pub extensible: bool,
}
