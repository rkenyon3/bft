//! Brainfuck program interpreter
//!
//! Creates a virtual machine with a memory tape of cells of type T, and can accept and (eventually)
//! run a program

use std::{error::Error, fmt::Display, num::NonZeroUsize};

use bft_types::{BfProgram, Instruction, LocalisedInstruction};

/// Represents a machine with a memory tape of cells. Accepts a type T for the tape
#[derive(Debug)]
pub struct VirtualMachine<T> {
    cells: Vec<T>,
    head: usize,
    tape_can_grow: bool,
    program_counter: usize,
    program: BfProgram,
}

pub trait CellKind: Clone + Default {
    /// Increment the given value, wrapping on overflow
    fn wrapping_increment(&mut self);
    /// Increment the given value, wrapping on underflow
    fn wrapping_decrement(&mut self);
    /// Sets the value of the cell
    fn set_value(&mut self, value: u8);
}

impl<T> VirtualMachine<T>
where
    T: Clone + Default + CellKind,
{
    /// Create a new VirtualMachine. Defaults to 30000 cells of memory if tape_size is zero.
    ///
    /// ```no_run
    ///# fn main() -> Result<(), Box<dyn std::error::Error>>{
    ///# use bft_types::BfProgram;
    ///# use bft_interp::VirtualMachine;
    ///# use std::num::NonZeroUsize;
    ///#
    /// let bf_program = BfProgram::from_file("my_bf_program.bf")?;
    ///
    /// let tape_size: Option::<NonZeroUsize> = Some(NonZeroUsize::new(30000).unwrap());
    /// let bf_interpreter: VirtualMachine<u8> = VirtualMachine::new(bf_program, tape_size, true);
    ///#
    ///# Ok(())
    ///# }
    /// ```
    pub fn new(program: BfProgram, tape_size: Option<NonZeroUsize>, tape_can_grow: bool) -> Self {
        let tape_size = tape_size.map(NonZeroUsize::get).unwrap_or(30_000);

        Self {
            cells: vec![T::default(); tape_size],
            head: 0,
            tape_can_grow,
            program,
            program_counter: 0,
        }
    }

    /// Interpret the program
    ///
    /// ```no_run
    ///# fn main() -> Result<(), Box<dyn std::error::Error>>{
    ///# use bft_types::BfProgram;
    ///# use bft_interp::VirtualMachine;
    ///# use std::num::NonZeroUsize;
    ///#
    /// let bf_program = BfProgram::from_file("my_bf_program.bf")?;
    ///
    /// let tape_size: Option::<NonZeroUsize> = Some(NonZeroUsize::new(30000).unwrap());
    /// let mut bf_interpreter: VirtualMachine<u8> = VirtualMachine::new(bf_program, tape_size, true);
    /// bf_interpreter.interpret_program()?;
    ///#
    ///# Ok(())
    ///# }
    /// ```   
    pub fn interpret_program(&mut self) -> Result<(), VMError> {
        while self.program_counter != self.program.instructions().len() {
            match self.program.instructions()[self.program_counter].instruction() {
                Instruction::MoveLeft => self.move_head_left()?,
                Instruction::MoveRight => self.move_head_right()?,
                Instruction::Increment => self.increment_cell(),
                Instruction::Decrement => self.decrement_cell(),
                _ => (),
            };
        }

        Ok(())
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
    /// with an auto-extending tape, 1000 more cells will be added. If not,
    /// the VM will be sad and will throw an error out.
    fn move_head_right(&mut self) -> Result<(), VMError> {
        if self.head == (self.cells.capacity() - 1) && self.tape_can_grow {
            let extra_tape = vec![T::default(); 1000];
            self.cells.extend(extra_tape);
        }

        self.head += 1;
        if self.head == self.cells.capacity() {
            let bad_instruction = self.program.instructions()[self.program_counter].clone();
            return Err(VMError::HeadOverrun(bad_instruction));
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
}

impl CellKind for u8 {
    fn wrapping_increment(&mut self) {
        if *self == u8::MAX {
            *self = u8::MIN;
        } else {
            *self += 1;
        }
    }

    fn wrapping_decrement(&mut self) {
        if *self == u8::MIN {
            *self = u8::MAX;
        } else {
            *self -= 1;
        }
    }

    fn set_value(&mut self, value:u8) {
        *self = value;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum VMError {
    /// The head ran off the start of the tape
    HeadUnderrun(LocalisedInstruction),
    /// The head ran off the end of the (non-auto-extending) tape
    HeadOverrun(LocalisedInstruction),
}

impl Error for VMError {}

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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn test_program() -> BfProgram {
        let mut test_file = NamedTempFile::new().unwrap();
        test_file.write_all(b"[test]+++.").unwrap();

        BfProgram::from_file(test_file.path()).unwrap()
    }

    // Does creating a VM with all paameters explicitly specified work?
    #[test]
    fn test_create_vm_explicit_params() {
        let test_program = test_program();
        let tape_size = Some(NonZeroUsize::new(10_000).unwrap());
        let vm: VirtualMachine<u8> = VirtualMachine::new(test_program, tape_size, true);

        assert_eq!(vm.cells.capacity(), 10_000);
        assert_eq!(vm.head, 0);
        assert_eq!(vm.tape_can_grow, true);
    }

    // Does creating a VM with a default tape size work?
    #[test]
    fn test_create_vm_default_params() {
        let test_program = test_program();
        let vm: VirtualMachine<u8> = VirtualMachine::new(test_program, None, true);

        assert_eq!(vm.cells.len(), 30_000);
        assert_eq!(vm.head, 0);
        assert_eq!(vm.tape_can_grow, true);
    }

    // Does moving the head left with space on an extensible tape work?
    #[test]
    fn test_move_head_left_extensible_good() {
        let test_program = test_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(test_program, None, true);
        vm.head = 5;
        assert!(vm.head == 5);

        let result = vm.move_head_left();
        let expected = Ok(());

        assert_eq!(result, expected);
        assert_eq!(vm.head, 4);
    }

    // Does moving the head left with space on an fixed tape work?
    #[test]
    fn test_move_head_left_fixed_good() {
        let test_program = test_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(test_program, None, false);
        vm.head = 5;
        assert!(vm.head == 5);

        let result = vm.move_head_left();
        let expected = Ok(());

        assert_eq!(result, expected);
        assert_eq!(vm.head, 4);
    }

    // Does moving the head left at the start of an extensible tape error correctly?
    #[test]
    fn test_move_head_left_extensible_bad() {
        let test_program = test_program();
        let bad_instruction = test_program.instructions()[0].clone();

        let mut vm: VirtualMachine<u8> = VirtualMachine::new(test_program, None, true);
        assert!(vm.head == 0);

        let result = vm.move_head_left();
        let expected = Err(VMError::HeadUnderrun(bad_instruction));

        assert_eq!(result, expected);
    }

    // Does moving the head left at the start of an fixed tape error correctly?
    #[test]
    fn test_move_head_left_fixed_bad() {
        let test_program = test_program();
        let bad_instruction = test_program.instructions()[0].clone();

        let mut vm: VirtualMachine<u8> = VirtualMachine::new(test_program, None, false);
        assert!(vm.head == 0);

        let result = vm.move_head_left();
        let expected = Err(VMError::HeadUnderrun(bad_instruction));

        assert_eq!(result, expected);
    }

    // Does moving the head right on an extensible tape work when the head has space to move?
    #[test]
    fn test_move_head_right_extensible_good() {
        let test_program = test_program();
        let tape_len = Some(NonZeroUsize::new(1000).unwrap());
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(test_program, tape_len, true);
        assert!(vm.head == 0);

        let result = vm.move_head_right();
        let expected = Ok(());

        assert_eq!(result, expected);
        assert_eq!(vm.head, 1);
    }

    // Does moving the head right on an fixed tape work?
    #[test]
    fn test_move_head_right_fixed_good() {
        let test_program = test_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(test_program, None, false);
        assert!(vm.head == 0);

        let result = vm.move_head_right();
        let expected = Ok(());

        assert_eq!(result, expected);
        assert_eq!(vm.head, 1);
    }

    // DOes moving the head right at the end of a fixed tape error correctly?
    #[test]
    fn test_move_head_right_fixed_bad() {
        let test_program = test_program();
        let bad_instruction = test_program.instructions()[0].clone();
        let tape_len = Some(NonZeroUsize::new(1000).unwrap());
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(test_program, tape_len, false);
        vm.head = 999;

        let result = vm.move_head_right();
        let expected = Err(VMError::HeadOverrun(bad_instruction));

        assert_eq!(result, expected);
    }

    // Does moving the head right at the end of an extensible tape make the tape grow?
    #[test]
    fn test_auto_tape_extension() {
        let test_program = test_program();
        let tape_len = Some(NonZeroUsize::new(1000).unwrap());
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(test_program, tape_len, true);

        vm.head = 999;
        assert!(vm.head == 999);

        let result = vm.move_head_right();
        let expected = Ok(());

        assert_eq!(result, expected);
        assert_eq!(vm.head, 1000);
        assert_eq!(vm.cells.capacity(), 2000)
    }

    // For u8, does incrementing without wrapping work?
    #[test]
    fn test_u8_increment_no_wrap() {
        let test_program = test_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(test_program, None, false);

        assert_eq!(vm.head, 0);

        vm.cells[0] = 10;
        vm.increment_cell();

        assert_eq!(vm.cells[0], 11);
    }

    // For u8, does incrementing wrap around the max value?
    #[test]
    fn test_u8_increment_wrap() {
        let test_program = test_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(test_program, None, false);

        assert_eq!(vm.head, 0);

        vm.cells[0] = u8::MAX;
        vm.increment_cell();

        assert_eq!(vm.cells[0], u8::MIN);
    }
    // For u8, does decrementing without wrapping work?
    #[test]
    fn test_u8_decrement_no_wrap() {
        let test_program = test_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(test_program, None, false);

        assert_eq!(vm.head, 0);

        vm.cells[0] = 10;
        vm.decrement_cell();

        assert_eq!(vm.cells[0], 9);
    }

    // For u8, does decrementing wrap around the min value?
    #[test]
    fn test_u8_decrement_wrap() {
        let test_program = test_program();
        let mut vm: VirtualMachine<u8> = VirtualMachine::new(test_program, None, false);

        assert_eq!(vm.head, 0);

        vm.cells[0] = u8::MIN;
        vm.decrement_cell();

        assert_eq!(vm.cells[0], u8::MAX);
    }
}
