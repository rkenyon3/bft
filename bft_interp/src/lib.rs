//! Brainfuck program interpreter
//!
//! Creates a [VirtualMachine] using parameters specified on the command line, and runs the
//! [BfProgram] it was given.

use std::{
    io::{Read, Write},
    num::NonZeroUsize,
};
use thiserror::Error;

use bft_types::{BfProgram, Instruction, LocalisedInstruction};

/// Error types that the [VirtualMachine] can emit. In all cases, the [VMError] includes details of
/// the [LocalisedInstruction] that caused it.
#[derive(Debug, PartialEq, Eq, Error)]
pub enum VMError {
    /// The head ran off the start of the tape. Note that the tape may never be extended at the start.
    #[error("Head underrun error occured at line {} column {}",.0.line_num(), .0.column_num())]
    HeadUnderrun(LocalisedInstruction),
    /// The head ran off the end of the (non-auto-extending) tape.
    #[error("Head overrun error occured at line {} column {}",.0.line_num(), .0.column_num())]
    HeadOverrun(LocalisedInstruction),
    /// Reading a byte from stdio failed. The text of the underlying IO error is included.
    #[error("Read error occured at line {} column {}: {}",.0.line_num(), .0.column_num(), .1)]
    ReadError(LocalisedInstruction, String),
    /// Writing a byte from stdio failed. The text of the underlying IO error is included.
    #[error("Write error occured at line {} column {}: {}",.0.line_num(), .0.column_num(), .1)]
    WriteError(LocalisedInstruction, String),
}

/// Represents a virtual machine with a memory tape of cells. Accepts a type T for the tape,
/// provided [CellKind] is implemented for T
#[derive(Debug)]
pub struct VirtualMachine<'a, T> {
    cells: Vec<T>,
    head: usize,
    tape_can_grow: bool,
    program_counter: usize,
    program: &'a BfProgram,
}

/// Trait requirements for the [VirtualMachine] tape cells
pub trait CellKind: Clone + Default {
    /// Increment the given value, wrapping on overflow
    fn wrapping_increment(&mut self);
    /// Increment the given value, wrapping on underflow
    fn wrapping_decrement(&mut self);
    /// Sets the value of the cell
    fn set_value(&mut self, value: u8);
    /// Gets the value of the cell
    fn get_value(&self) -> u8;
    /// Determine if the value of the cell is zero
    fn is_zero(&self) -> bool;
}

impl<'a, T> VirtualMachine<'a, T>
where
    T: CellKind,
{
    /// Create a new VirtualMachine. Defaults to 30000 cells of memory if tape_size is zero.
    ///
    /// ```
    ///# fn main() -> Result<(), Box<dyn std::error::Error>>{
    ///# use bft_types::BfProgram;
    ///# use bft_interp::VirtualMachine;
    ///# use std::num::NonZeroUsize;
    ///#
    /// let mut bf_program = BfProgram::new("my_file.bf",".>.>+++")?;
    ///
    /// let tape_size: Option::<NonZeroUsize> = Some(NonZeroUsize::new(50000).unwrap());
    /// let bf_interpreter: VirtualMachine<u8> = VirtualMachine::new(&bf_program, tape_size, true);
    ///#
    ///# Ok(())
    ///# }
    /// ```
    pub fn new(
        program: &'a BfProgram,
        tape_size: Option<NonZeroUsize>,
        tape_can_grow: bool,
    ) -> Self {
        let tape_size = tape_size.map(NonZeroUsize::get).unwrap_or(30_000);

        Self {
            cells: vec![T::default(); tape_size],
            head: 0,
            tape_can_grow,
            program,
            program_counter: 0,
        }
    }

    /// Interprets the [BfProgram] the machine was instantiated with.
    ///
    /// ```
    ///# fn main() -> Result<(), Box<dyn std::error::Error>>{
    ///# use bft_types::BfProgram;
    ///# use bft_interp::VirtualMachine;
    ///# use std::num::NonZeroUsize;
    ///# use std::io::{stdin, stdout};
    ///#
    /// let mut bf_program = BfProgram::new("my_file.bf",".>.>+++")?;
    ///
    /// let mut bf_interpreter: VirtualMachine<u8> = VirtualMachine::new(&bf_program, None, true);
    /// bf_interpreter.interpret(&mut stdin(), &mut stdout())?;
    ///#
    ///# Ok(())
    ///# }
    /// ```   
    pub fn interpret(
        &mut self,
        input: &mut impl Read,
        output: &mut impl Write,
    ) -> Result<(), VMError> {
        while self.program_counter < self.program.localised_instructions().len() {
            self.program_counter =
                match self.program.localised_instructions()[self.program_counter].instruction() {
                    Instruction::MoveLeft => self.move_head_left()?,
                    Instruction::MoveRight => self.move_head_right()?,
                    Instruction::Increment => self.increment_cell()?,
                    Instruction::Decrement => self.decrement_cell()?,
                    Instruction::Input => self.read_value(input)?,
                    Instruction::Output => self.print_value(output)?,
                    Instruction::ConditionalJumpForward => self.conditional_jump_forward()?,
                    Instruction::ConditionalJumpBackward => self.conditional_jump_backward()?,
                };
        }
        Ok(())
    }

    /// Move the head one cell towards the left (start) of the tape
    fn move_head_left(&mut self) -> Result<usize, VMError> {
        if self.head > 0 {
            // note: went with this over checked_sub
            self.head -= 1;

            Ok(self.program_counter + 1)
        } else {
            let bad_instruction = self.program.localised_instructions()[self.program_counter];
            Err(VMError::HeadUnderrun(bad_instruction))
        }
    }

    /// Move the head one cell towards the right (end) of the tape.
    /// If the head is at the end of the tape and the VM has been instantiated
    /// with an auto-extending tape, more cells will be added. If not, the VM
    /// will be sad and will throw an error out.
    fn move_head_right(&mut self) -> Result<usize, VMError> {
        self.head += 1;

        if self.head == self.cells.len() {
            if self.tape_can_grow {
                self.cells.push(T::default());
            } else {
                let bad_instruction = self.program.localised_instructions()[self.program_counter];
                return Err(VMError::HeadOverrun(bad_instruction));
            }
        }

        Ok(self.program_counter + 1)
    }

    /// Perform a wrapping increment on the cell pointed at by the head
    fn increment_cell(&mut self) -> Result<usize, VMError> {
        self.cells[self.head].wrapping_increment();
        Ok(self.program_counter + 1)
    }

    /// Perform a wrapping decrement on the cell pointed at by the head
    fn decrement_cell(&mut self) -> Result<usize, VMError> {
        self.cells[self.head].wrapping_decrement();
        Ok(self.program_counter + 1)
    }

    /// Read a single byte from [source] and write it to the cell at head
    fn read_value(&mut self, source: &mut impl Read) -> Result<usize, VMError> {
        let mut buffer = [0];
        match source.read_exact(&mut buffer) {
            Ok(_) => {
                self.cells[self.head].set_value(buffer[0]);
                Ok(self.program_counter + 1)
            }
            Err(error) => {
                let bad_instruction = self.program.localised_instructions()[self.program_counter];
                Err(VMError::from((bad_instruction, error)))
            }
        }
    }

    /// Print the value at head to the target output
    fn print_value(&self, output: &mut impl Write) -> Result<usize, VMError> {
        let output_buf = [self.cells[self.head].get_value()];
        output
            .write_all(&output_buf)
            .and_then(|_| output.flush())
            .map(|_| &self.program_counter + 1)
            .map_err(|error| {
                let bad_instruction = self.program.localised_instructions()[self.program_counter];
                VMError::from((bad_instruction, error))
            })
    }

    /// Get the next program instruction index based on the value of the cell under the head.
    /// If the cell is zero, return the index of the instruction after the matching ].
    /// If the cell is not zero, return the index of the next instruction after this one.
    fn conditional_jump_forward(&self) -> Result<usize, VMError> {
        if self.cells[self.head].is_zero() {
            return Ok(self.program.jump_target(self.program_counter));
        }
        Ok(self.program_counter + 1)
    }

    /// Get the next program instruction index based on the value of the cell under the head.
    /// If the cell is zero, return the index of the next instruction after this one.
    /// If the cell is not zero, return the index of the instruction after the matching [.
    fn conditional_jump_backward(&self) -> Result<usize, VMError> {
        if self.cells[self.head].is_zero() {
            return Ok(self.program_counter + 1);
        }
        Ok(self.program.jump_target(self.program_counter))
    }
}

impl CellKind for u8 {
    fn wrapping_increment(&mut self) {
        *self = self.wrapping_add(1);
    }

    fn wrapping_decrement(&mut self) {
        *self = self.wrapping_sub(1);
    }

    fn set_value(&mut self, value: u8) {
        *self = value;
    }

    fn get_value(&self) -> u8 {
        *self
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl From<(LocalisedInstruction, std::io::Error)> for VMError {
    fn from(value: (LocalisedInstruction, std::io::Error)) -> Self {
        let bad_instruction = value.0;
        let error_msg = value.1.to_string();
        if bad_instruction.instruction() == Instruction::Input {
            VMError::ReadError(bad_instruction, error_msg)
        } else {
            VMError::WriteError(bad_instruction, error_msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    fn make_placeholder_program() -> BfProgram {
        BfProgram::new("test_file.bf", ",.[test]+++.").unwrap()
    }

    // Does creating a VM with all paameters explicitly specified work?
    #[test]
    fn test_create_vm_explicit_params() {
        let placeholder_program = make_placeholder_program();
        let test_program = placeholder_program.clone();
        let tape_size = Some(NonZeroUsize::new(10_000).unwrap());
        let vm: VirtualMachine<u8> = VirtualMachine::new(&placeholder_program, tape_size, true);

        assert_eq!(vm.cells.len(), 10_000);
        assert_eq!(vm.head, 0);
        assert!(vm.tape_can_grow);
        assert_eq!(vm.program_counter, 0);
        assert_eq!(*vm.program, test_program);
    }

    // Does creating a VM with a default tape size work?
    #[test]
    fn test_create_vm_default_params() {
        let placeholder_program = make_placeholder_program();

        let vm: VirtualMachine<u8> = VirtualMachine::new(&placeholder_program, None, true);

        assert_eq!(vm.cells.len(), 30_000);
    }

    // Does moving the head left work?
    #[test]
    fn test_move_head_left_extensible_good() {
        let test_program = make_placeholder_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, None, true);
        vm.head = 5;

        let result = vm.move_head_left();

        assert!(result.is_ok());
        assert_eq!(vm.head, 4);
    }

    // Does moving the head left at the start of the tape error correctly?
    #[test]
    fn test_move_head_left_extensible_bad() {
        let test_program = make_placeholder_program();
        let bad_instruction = test_program.localised_instructions()[0];

        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, None, true);

        let result = vm.move_head_left();
        let expected_error: Result<usize, VMError> = Err(VMError::HeadUnderrun(bad_instruction));

        assert!(result.is_err());
        assert_eq!(result, expected_error);
    }

    // Does moving the head right on an extensible tape work when the head has space to move?
    #[test]
    fn test_move_head_right_extensible_good() {
        let test_program = make_placeholder_program();
        let tape_len = Some(NonZeroUsize::new(1000).unwrap());
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, tape_len, true);

        let result = vm.move_head_right();

        assert!(result.is_ok());
        assert_eq!(vm.head, 1);
    }

    // Does moving the head right on an fixed tape work?
    #[test]
    fn test_move_head_right_fixed_good() {
        let test_program = make_placeholder_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, None, false);

        let result = vm.move_head_right();

        assert!(result.is_ok());
        assert_eq!(vm.head, 1);
    }

    // Does moving the head right at the end of a fixed tape error correctly?
    #[test]
    fn test_move_head_right_fixed_bad() {
        let test_program = make_placeholder_program();
        let bad_instruction = test_program.localised_instructions()[0];
        let tape_len = Some(NonZeroUsize::new(1000).unwrap());
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, tape_len, false);
        vm.head = 999;

        let result = vm.move_head_right();
        let expected_error: Result<usize, VMError> = Err(VMError::HeadOverrun(bad_instruction));

        assert!(result.is_err());
        assert_eq!(result, expected_error);
    }

    // Does moving the head right at the end of an extensible tape make the tape grow?
    #[test]
    fn test_auto_tape_extension() {
        let test_program = make_placeholder_program();
        let tape_len = Some(NonZeroUsize::new(1000).unwrap());
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, tape_len, true);

        vm.head = 999;

        let result = vm.move_head_right();

        assert!(result.is_ok());
        assert_eq!(vm.head, 1000);
        assert_eq!(vm.cells.len(), 1001);
    }

    // For u8, does incrementing without wrapping work?
    #[test]
    fn test_u8_increment_no_wrap() {
        let test_program = make_placeholder_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, None, false);

        vm.cells[0] = 10;
        let _ = vm.increment_cell();

        assert_eq!(vm.cells[0], 11);
    }

    // For u8, does incrementing wrap around the max value?
    #[test]
    fn test_u8_increment_wrap() {
        let test_program = make_placeholder_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, None, false);

        vm.cells[0] = u8::MAX;
        let _ = vm.increment_cell();

        assert_eq!(vm.cells[0], u8::MIN);
    }
    // For u8, does decrementing without wrapping work?
    #[test]
    fn test_u8_decrement_no_wrap() {
        let test_program = make_placeholder_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, None, false);

        vm.cells[0] = 10;
        let _ = vm.decrement_cell();

        assert_eq!(vm.cells[0], 9);
    }

    // For u8, does decrementing wrap around the min value?
    #[test]
    fn test_u8_decrement_wrap() {
        let test_program = make_placeholder_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, None, false);

        vm.cells[0] = u8::MIN;
        let _ = vm.decrement_cell();

        assert_eq!(vm.cells[0], u8::MAX);
    }

    // does reading a byte into a cell work?
    #[test]
    fn test_read() {
        let test_program = make_placeholder_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, None, false);
        let mut cursor = std::io::Cursor::new(vec![1, 2, 3]);

        let result = vm.read_value(&mut cursor);

        assert!(result.is_ok());
        assert_eq!(vm.cells[0], 1);
    }

    // does reading error when the buffer has nothing to read
    #[test]
    fn test_read_bad() {
        let test_program = make_placeholder_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, None, false);
        let mut cursor = std::io::Cursor::new([0; 0]); // zero-length buffer to break the thing

        let result = vm.read_value(&mut cursor);

        assert!(result.is_err());
    }

    // does writing a byte from a cell work?
    #[test]
    fn test_write() {
        let test_program = make_placeholder_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, None, false);
        let buf: Vec<u8> = vec![0; 10];
        let mut cursor = std::io::Cursor::new(buf);

        vm.cells[0] = 65;

        let result = vm.print_value(&mut cursor);

        assert!(result.is_ok());
        assert_eq!(cursor.get_ref()[0], 65);
    }

    // does reading error correctly when the output has no space?
    #[test]
    fn test_write_bad() {
        let test_program = make_placeholder_program();
        let vm: VirtualMachine<u8> = VirtualMachine::new(&test_program, None, false);
        let mut cursor = std::io::Cursor::new([0; 0]);

        let result = vm.print_value(&mut cursor);

        assert!(result.is_err());
    }

    // Helper function for testing jumps
    fn jumps_test_program() -> BfProgram {
        let test_program_content = "[..]..";
        BfProgram::new("test_program.bf", test_program_content).unwrap()
    }

    // Does a conditional jump forward work when the cell under the head is zero
    #[test]
    fn test_forward_jump_cell_zero() {
        let prog = jumps_test_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&prog, None, true);

        vm.cells[0].set_value(0);
        vm.program_counter = 0;

        let next_prog_index = vm.conditional_jump_forward().unwrap();

        assert_eq!(next_prog_index, 4)
    }

    // Does a conditional jump forward work when the cell under the head is not zero
    #[test]
    fn test_forward_jump_cell_nonzero() {
        let prog = jumps_test_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&prog, None, true);

        vm.cells[0].set_value(7);
        vm.program_counter = 0;

        let next_prog_index = vm.conditional_jump_forward().unwrap();

        assert_eq!(next_prog_index, 1)
    }

    // Does a conditional jump backward work when the cell under the head is zero
    #[test]
    fn test_backward_jump_cell_zero() {
        let prog = jumps_test_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&prog, None, true);

        vm.cells[0].set_value(0);
        vm.program_counter = 3;

        let next_prog_index = vm.conditional_jump_backward().unwrap();

        assert_eq!(next_prog_index, 4)
    }

    // Does a conditional jump backward work when the cell under the head is not zero
    #[test]
    fn test_backward_jump_cell_nonzero() {
        let prog = jumps_test_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&prog, None, true);

        vm.cells[0].set_value(7);
        vm.program_counter = 3;

        let next_prog_index = vm.conditional_jump_backward().unwrap();

        assert_eq!(next_prog_index, 1)
    }

    // run a hello world test program
    #[test]
    fn test_hello_world() {
        let prog_contents = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
        let program = BfProgram::new("hello_world.bf", prog_contents).unwrap();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&program, None, false);

        let mut input_cursor: Cursor<[u8; 10]> = Cursor::new([0; 10]);
        let mut output_cursor: Cursor<[u8; 15]> = Cursor::new([0; 15]);

        vm.interpret(&mut input_cursor, &mut output_cursor).unwrap();

        let expected = [
            b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r', b'l', b'd', b'!', b'\n', 0, 0,
        ];
        let actual = output_cursor.into_inner();
        assert_eq!(expected, actual);
    }
}
