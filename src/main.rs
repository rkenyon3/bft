use std::env::{self, Args};
use std::path::PathBuf;
use bft_types::BfProgram;
use bft_interp::VirtualMachine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = PathBuf::from(env::args().nth(1).ok_or("Needs BF program file name")?);

    let bf_program = BfProgram::from_file(file_path.as_path())?;
    let bf_interpreter:VirtualMachine<u8> = VirtualMachine::new(30000, false);
    
    Ok(())
}
