//! Brainfuck Interpreter. Creates a [BfProgram] from a BrainFuck program file and runs it on a
//! [VirtualMachine]. The program is first analysed to confirm that the jump commands ('[' and ']')
//! are balanced.
//!
//! The virtual machine contains a tape of cells that can be moved under a read/write head. The
//! size of this tape may be specified as --cells cell_count, or will default to 30,000.
//!
//! The virtual machine is connected to stdin and stdout

mod cli;

use std::{io::Write, process::ExitCode};

use bft_interp::VirtualMachine;
use bft_types::BfProgram;
use clap::Parser;
use std::io::{stdin, stdout};

use cli::Args;

/// Ensures the output that it writes has a newline at the end.
/// If the program doesn't produce one, this will add it.
struct WriterWithTrailingNewline<'a, T: Write> {
    /// Anything implementing Write. All output will be passed to this.
    inner_writer: &'a mut T,
    /// Records the last byte that was written after each write.
    last_byte: u8,
}

impl<'a, T: Write> WriterWithTrailingNewline<'a, T> {
    /// Creates a new instance of this struct.
    fn new(inner_writer: &'a mut T) -> Self {
        Self {
            inner_writer,
            last_byte: 0,
        }
    }
}

impl<'a, T: Write> Write for WriterWithTrailingNewline<'a, T> {
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

impl<'a, T: Write> Drop for WriterWithTrailingNewline<'a, T> {
    fn drop(&mut self) {
        if self.last_byte != b'\n' {
            writeln!(self.inner_writer).expect("Failed to write newline");
        }
    }
}

/// Create a [BfProgram] from the file specified, then construct a [VirtualMachine] and run it.
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
    let mut output = stdout();
    let mut output_with_newline = WriterWithTrailingNewline::new(&mut output);
    bf_interpreter.interpret(&mut input, &mut output_with_newline)?;

    Ok(())
}

/// Main function. Returns a success code if everything worked, or an error and prints an error message if it didn't
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
    fn test_output_with_newline() {
        let prog_contents = ",.,."; // simply take one byte and echo it back
        let program = BfProgram::new("echo.bf", prog_contents).unwrap();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&program, None, false);

        let mut input_cursor = Cursor::new([b'A', b'\n']);
        let mut output_cursor = Cursor::new([0; 3]);

        // put the output_writer and vm run in a block so the output_writer is dropped at the end
        // and we can get at the output_cursor
        {
            let mut output_writer = WriterWithTrailingNewline::new(&mut output_cursor);

            vm.interpret(&mut input_cursor, &mut output_writer).unwrap();
        }

        let expected = [b'A', b'\n', 0];
        let actual = output_cursor.into_inner();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_output_without_newline() {
        let prog_contents = ",.,."; // simply take one byte and echo it back
        let program = BfProgram::new("echo.bf", prog_contents).unwrap();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&program, None, false);

        let mut input_cursor = Cursor::new([b'A', b'B']);
        let mut output_cursor = Cursor::new([0; 3]);

        // put the output_writer and vm run in a block so the output_writer is dropped at the end
        // and we can get at the output_cursor
        {
            let mut output_writer = WriterWithTrailingNewline::new(&mut output_cursor);

            vm.interpret(&mut input_cursor, &mut output_writer).unwrap();
        }

        let expected = [b'A', b'B', b'\n'];
        let actual = output_cursor.into_inner();
        assert_eq!(expected, actual);
    }
}
