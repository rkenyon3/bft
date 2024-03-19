//! Brainfuck program interpreter
//!
//! Creates a [VirtualMachine] using parameters specified on the command line, and runs the
//! [BfProgram] it was given.

use std::{
    fmt::Display,
    io::{Read, Write},
    num::NonZeroUsize,
};

use bft_types::{BfProgram, Instruction, LocalisedInstruction};

/// Error types that the [VirtualMachine] can emit
#[derive(Debug, PartialEq, Eq)]
pub enum VMError {
    /// The head ran off the start of the tape
    HeadUnderrun(LocalisedInstruction),
    /// The head ran off the end of the (non-auto-extending) tape
    HeadOverrun(LocalisedInstruction),
    /// Reading a byte from stdio went bloop
    ReadError(LocalisedInstruction, String),
    /// Writing a byte from stdio went bloop
    WriteError(LocalisedInstruction, String),
}

/// Represents a machine with a memory tape of cells. Accepts a type T for the tape
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
    /// let mut bf_program = BfProgram::new("my_file.bf",".>.>+++");
    ///
    /// let tape_size: Option::<NonZeroUsize> = Some(NonZeroUsize::new(30000).unwrap());
    /// let bf_interpreter: VirtualMachine<u8> = VirtualMachine::new(&mut bf_program, tape_size, true)?;
    ///#
    ///# Ok(())
    ///# }
    /// ```
    pub fn new(
        program: &'a mut BfProgram,
        tape_size: Option<NonZeroUsize>,
        tape_can_grow: bool,
    ) -> Result<Self, String> {
        let tape_size = tape_size.map(NonZeroUsize::get).unwrap_or(30_000);

        // enforce analysing the program, since we need the jump map
        program.analyse_program()?;

        Ok(Self {
            cells: vec![T::default(); tape_size],
            head: 0,
            tape_can_grow,
            program,
            program_counter: 0,
        })
    }

    /// Interpret the program
    ///
    /// ```
    ///# fn main() -> Result<(), Box<dyn std::error::Error>>{
    ///# use bft_types::BfProgram;
    ///# use bft_interp::VirtualMachine;
    ///# use std::num::NonZeroUsize;
    ///#
    /// let mut bf_program = BfProgram::new("my_file.bf",".>.>+++");
    ///
    /// let tape_size: Option::<NonZeroUsize> = Some(NonZeroUsize::new(10000).unwrap());
    /// let mut bf_interpreter: VirtualMachine<u8> = VirtualMachine::new(&mut bf_program, tape_size, true)?;
    /// bf_interpreter.interpret_program()?;
    ///#
    ///# Ok(())
    ///# }
    /// ```   
    pub fn interpret_program(&mut self) -> Result<(), VMError> {
        while self.program_counter != self.program.instructions().len() {
            let next_head_position = self.interpret_instruction()?;
            self.head = next_head_position;
        }

        Ok(())
    }

    fn interpret_instruction(&mut self) -> Result<usize, VMError> {
        match self.program.instructions()[self.program_counter].instruction() {
            Instruction::MoveLeft => self.move_head_left()?,
            Instruction::MoveRight => self.move_head_right()?,
            Instruction::Increment => self.increment_cell(),
            Instruction::Decrement => self.decrement_cell(),
            Instruction::Input => self.read_value(&mut std::io::stdin())?,
            Instruction::Output => self.print_value(&mut std::io::stdout())?,
            Instruction::ConditionalJumpForward => todo!(),
            Instruction::ConditionalJumpBackward => todo!(),
        };

        Ok(self.head + 1)
    }

    /// Move the head one cell towards the left (start) of the tape
    fn move_head_left(&mut self) -> Result<(), VMError> {
        if self.head > 0 {
            self.head -= 1;

            Ok(())
        } else {
            let bad_instruction = self.program.instructions()[self.program_counter].clone();
            Err(VMError::HeadUnderrun(bad_instruction))
        }
    }

    /// Move the head one cell towards the right (end) of the tape.
    /// If the head is at the end of the tape and the VM has been instantiated
    /// with an auto-extending tape, more cells will be added. If not, the VM
    /// will be sad and will throw an error out.
    fn move_head_right(&mut self) -> Result<(), VMError> {
        self.head += 1;

        if self.head == self.cells.len() {
            if self.tape_can_grow {
                self.cells.push(T::default());
            } else {
                let bad_instruction = self.program.instructions()[self.program_counter].clone();
                return Err(VMError::HeadOverrun(bad_instruction));
            }
        }

        Ok(())
    }

    /// Perform a wrapping increment on the cell pointed at by the head
    fn increment_cell(&mut self) {
        self.cells[self.head].wrapping_increment();
    }

    /// Perform a wrapping decrement on the cell pointed at by the head
    fn decrement_cell(&mut self) {
        self.cells[self.head].wrapping_decrement();
    }

    /// Read a single byte from [source] and write it to the cell at head
    fn read_value(&mut self, source: &mut impl Read) -> Result<(), VMError> {
        let mut buffer = [0];
        match source.read_exact(&mut buffer) {
            Ok(_) => {
                self.cells[self.head].set_value(buffer[0]);
                Ok(())
            }
            Err(error) => {
                let bad_instruction = self.program.instructions()[self.program_counter].clone();
                Err(VMError::from((bad_instruction, error)))
            }
        }
    }

    /// Print the value at head to the target output
    fn print_value(&self, output: &mut impl Write) -> Result<(), VMError> {
        let output_buf = [self.cells[self.head].get_value()];
        match output.write_all(&output_buf) {
            Ok(_) => Ok(()),

            Err(error) => {
                let bad_instruction = self.program.instructions()[self.program_counter].clone();
                Err(VMError::from((bad_instruction, error)))
            }
        }
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
}

impl std::error::Error for VMError {}

impl Display for VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HeadOverrun(program_instruction) => {
                write!(
                    f,
                    "Head Overrun Error occured at line {} column {}",
                    program_instruction.line_num(),
                    program_instruction.column_num()
                )
            }
            Self::HeadUnderrun(program_instruction) => {
                write!(
                    f,
                    "Head Underrun Error occured at line {} column {}",
                    program_instruction.line_num(),
                    program_instruction.column_num()
                )
            }
            Self::ReadError(program_instruction, error) => {
                write!(
                    f,
                    "Read Error occured at line {} column {}: {}",
                    program_instruction.line_num(),
                    program_instruction.column_num(),
                    error
                )
            }
            Self::WriteError(program_instruction, error) => {
                write!(
                    f,
                    "Write Error occured at line {} column {}: {}",
                    program_instruction.line_num(),
                    program_instruction.column_num(),
                    error
                )
            }
        }
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
    use super::*;
    use std::io::{ErrorKind, Write};
    use tempfile::NamedTempFile;

    fn make_palceholder_program() -> BfProgram {
        BfProgram::new("test_file.bf", ",.[test]+++.")
    }

    // Does creating a VM with all paameters explicitly specified work?
    #[test]
    fn test_create_vm_explicit_params() {
        let mut placeholder_program = make_palceholder_program();
        let test_program = placeholder_program.clone();
        let tape_size = Some(NonZeroUsize::new(10_000).unwrap());
        let vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut placeholder_program, tape_size, true).unwrap();

        assert_eq!(vm.cells.len(), 10_000);
        assert_eq!(vm.head, 0);
        assert_eq!(vm.tape_can_grow, true);
        assert_eq!(vm.program_counter, 0);
        let test_program = test_program.clone();
        assert_eq!(vm.program, &test_program);
    }

    // Does creating a VM with a default tape size work?
    #[test]
    fn test_create_vm_default_params() {
        let mut placeholder_program = make_palceholder_program();

        let vm: VirtualMachine<u8> = VirtualMachine::new(&mut placeholder_program, None, true).unwrap();

        assert_eq!(vm.cells.len(), 30_000);
    }

    // Does moving the head left work?
    #[test]
    fn test_move_head_left_extensible_good() {
        let mut test_program = make_palceholder_program();
        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, None, true).unwrap();
        vm.head = 5;

        let result = vm.move_head_left();

        assert!(result.is_ok());
        assert_eq!(vm.head, 4);
    }

    // Does moving the head left at the start of the tape error correctly?
    #[test]
    fn test_move_head_left_extensible_bad() {
        let mut test_program = make_palceholder_program();
        let bad_instruction = test_program.instructions()[0].clone();

        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, None, true).unwrap();

        let result = vm.move_head_left();
        let expected_error: Result<(), VMError> = Err(VMError::HeadUnderrun(bad_instruction));

        assert!(result.is_err());
        assert_eq!(result, expected_error);
    }

    // Does moving the head right on an extensible tape work when the head has space to move?
    #[test]
    fn test_move_head_right_extensible_good() {
        let mut test_program = make_palceholder_program();
        let tape_len = Some(NonZeroUsize::new(1000).unwrap());
        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, tape_len, true).unwrap();

        let result = vm.move_head_right();

        assert!(result.is_ok());
        assert_eq!(vm.head, 1);
    }

    // Does moving the head right on an fixed tape work?
    #[test]
    fn test_move_head_right_fixed_good() {
        let mut test_program = make_palceholder_program();
        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, None, false).unwrap();

        let result = vm.move_head_right();

        assert!(result.is_ok());
        assert_eq!(vm.head, 1);
    }

    // Does moving the head right at the end of a fixed tape error correctly?
    #[test]
    fn test_move_head_right_fixed_bad() {
        let mut test_program = make_palceholder_program();
        let bad_instruction = test_program.instructions()[0].clone();
        let tape_len = Some(NonZeroUsize::new(1000).unwrap());
        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, tape_len, false).unwrap();
        vm.head = 999;

        let result = vm.move_head_right();
        let expected_error: Result<(), VMError> = Err(VMError::HeadOverrun(bad_instruction));

        assert!(result.is_err());
        assert_eq!(result, expected_error);
    }

    // Does moving the head right at the end of an extensible tape make the tape grow?
    #[test]
    fn test_auto_tape_extension() {
        let mut test_program = make_palceholder_program();
        let tape_len = Some(NonZeroUsize::new(1000).unwrap());
        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, tape_len, true).unwrap();

        vm.head = 999;

        let result = vm.move_head_right();

        assert!(result.is_ok());
        assert_eq!(vm.head, 1000);
        assert_eq!(vm.cells.len(), 1001);
    }

    // For u8, does incrementing without wrapping work?
    #[test]
    fn test_u8_increment_no_wrap() {
        let mut test_program = make_palceholder_program();
        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, None, false).unwrap();

        vm.cells[0] = 10;
        vm.increment_cell();

        assert_eq!(vm.cells[0], 11);
    }

    // For u8, does incrementing wrap around the max value?
    #[test]
    fn test_u8_increment_wrap() {
        let mut test_program = make_palceholder_program();
        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, None, false).unwrap();

        vm.cells[0] = u8::MAX;
        vm.increment_cell();

        assert_eq!(vm.cells[0], u8::MIN);
    }
    // For u8, does decrementing without wrapping work?
    #[test]
    fn test_u8_decrement_no_wrap() {
        let mut test_program = make_palceholder_program();
        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, None, false).unwrap();

        vm.cells[0] = 10;
        vm.decrement_cell();

        assert_eq!(vm.cells[0], 9);
    }

    // For u8, does decrementing wrap around the min value?
    #[test]
    fn test_u8_decrement_wrap() {
        let mut test_program = make_palceholder_program();
        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, None, false).unwrap();

        vm.cells[0] = u8::MIN;
        vm.decrement_cell();

        assert_eq!(vm.cells[0], u8::MAX);
    }

    // does reading a byte into a cell work?
    #[test]
    fn test_read() {
        let mut test_program = make_palceholder_program();
        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, None, false).unwrap();
        let mut cursor = std::io::Cursor::new(vec![1, 2, 3]);

        let result = vm.read_value(&mut cursor);

        assert!(result.is_ok());
        assert_eq!(vm.cells[0], 1);
    }

    // does reading error correctly when it should?
    #[test]
    fn test_read_bad() {
        let mut test_program = make_palceholder_program();
        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, None, false).unwrap();
        let mut cursor = std::io::Cursor::new([0; 0]); // zero-length buffer to break the thing

        let result = vm.read_value(&mut cursor);

        assert!(result.is_err());
    }

    // does reading a byte into a cell work?
    #[test]
    fn test_write() {
        let mut test_program = make_palceholder_program();
        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, None, false).unwrap();
        let mut buf: Vec<u8> = vec![0; 10];
        let mut cursor = std::io::Cursor::new(buf);

        vm.cells[0] = 65;

        let result = vm.print_value(&mut cursor);

        assert!(result.is_ok());
        assert_eq!(cursor.get_ref()[0], 65);
    }

    // does reading error correctly when it should?
    #[test]
    fn test_write_bad() {
        let mut test_program = make_palceholder_program();
        let mut vm: VirtualMachine<u8> =
            VirtualMachine::new(&mut test_program, None, false).unwrap();
        let mut cursor = std::io::Cursor::new([0; 0]);

        let result = vm.print_value(&mut cursor);

        assert!(result.is_err());
    }
}
