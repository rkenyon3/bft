//! Brainfuck Interpreter. This takes created a [BfProgram] from a file and runs it on a
//! virtual machine. The program is first analysed to confirm that the jump commands ('[' and ']')
//! are balanced.
//!
//! The virtual machine contains a tape of cells that can be under a read/write head. The size of
//! this tape may be specified as --cells cell_count, or will default to 30,000.
//!
//! The virtual machine input and output may be from stdin and stdout, or be specified as files
//! using --input file_name and --output file_name

mod cli;

use std::{io::Write, process::ExitCode};

use bft_interp::VirtualMachine;
use bft_types::BfProgram;
use clap::Parser;
use std::io::{stdin, stdout};

use cli::Args;

/// Writer that ensures the output that it writes has a newline at the end.
/// If the program doesn't produce one, this will add it.
struct WriterWithTrailingNewline<T: Write> {
    inner_writer: T,
    last_byte: u8,
}

impl<T: Write> WriterWithTrailingNewline<T> {
    fn new(inner_writer: T) -> Self {
        Self {
            inner_writer,
            last_byte: 0,
        }
    }
}

impl<T: Write> Write for WriterWithTrailingNewline<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(last_byte) = buf.last() {
            self.last_byte = *last_byte;
        }
        self.inner_writer.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner_writer.flush()
    }
}

impl<T: Write> Drop for WriterWithTrailingNewline<T> {
    fn drop(&mut self) {
        if self.last_byte != b'\n' {
            writeln!(self.inner_writer).expect("Failed to write newline");
        }
    }
}

/// Analyse the program for validity, then construct a [VirtualMachine] and
/// run it
///```no_run
/// let args = cli::Args::parse();
///
/// run_bft(&args)?;
///```
fn run_bft(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let bf_program = BfProgram::from_file(&args.program)?;

    let mut bf_interpreter: VirtualMachine<u8> =
        VirtualMachine::new(&bf_program, args.cells, args.extensible);

    let mut input = stdin();
    let mut output = WriterWithTrailingNewline::new(stdout());
    bf_interpreter.interpret(&mut input, &mut output)?;

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


#[cfg(test)]
mod tests {
    use super::*;    
    use std::io::Cursor;

    #[test]
    fn test_output_with_newline(){
        let prog_contents = ",."; // simply take one byte and echo it back
        let mut program = BfProgram::new("echo.bf", prog_contents).unwrap();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&mut program, None, false);

        let mut input_cursor = Cursor::new([b'A', b'\n']);
        let mut output_cursor = Cursor::new([0; 3]);
        {
            let mut output_writer = WriterWithTrailingNewline::new(output_cursor);
        
        let _ = vm.interpret(&mut input_cursor, &mut output_writer).unwrap();
        }
        let expected = [
            b'A', b'\n', 0,
        ];
        let actual = output_cursor.into_inner();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_xmas(){

    }
}