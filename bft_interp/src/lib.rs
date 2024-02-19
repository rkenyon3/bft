//! Brainfuck program interpreter
//!
//! Creates a virtual machine with a memory tape of cells of type T, and can accept and (eventually)
//! run a program

use std::num::NonZeroUsize;

use bft_types::{BfProgram, LocalisedInstruction};

/// Represents a machine with a memory tape of cells. Accepts a type T for the tape
#[derive(Debug)]
pub struct VirtualMachine<'a, T> {
    cells: Vec<T>,
    head: usize,
    tape_can_grow: bool,
    program_counter: usize,
    program: &'a BfProgram,
}

impl<'a, T> VirtualMachine<'a, T>
where
    T: Clone + Default,
{
    /// Create a new VirtualMachine. Defaults to 30000 cells of memory if tape_size is zero.
    ///
    /// ```no_run
    /// fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use bft_types::BfProgram;
    /// use bft_interp::VirtualMachine;
    /// use std::num::NonZeroUsize;
    ///
    /// let bf_program = BfProgram::from_file("my_bf_program.bf")?;
    ///
    /// let tape_size: Option::<NonZeroUsize> = Some(NonZeroUsize::new(30000).unwrap());
    /// let bf_interpreter: VirtualMachine<u8> = VirtualMachine::new(&bf_program, tape_size, true);
    ///
    /// Ok(())
    /// }
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

    /// Interpret the program
    pub fn interpret_program(&mut self) -> Result<(), VMError> {
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
        if self.head < self.cells.len() {
            self.head += 1;
            Ok(())
        } else {
            if self.tape_can_grow {
                self.cells.reserve_exact(1000);
                Ok(())
            } else {
                let bad_instruction = self.program.instructions()[self.program_counter].clone();
                Err(VMError::HeadOverrun(bad_instruction))
            }
        }
    }
}

pub enum VMError {
    /// The head ran off the end of the tape
    HeadOverrun(LocalisedInstruction),
    /// The head ran off the start of the tape
    HeadUnderrun(LocalisedInstruction),
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: implement
    // can probably use std::io::cursor here
}
