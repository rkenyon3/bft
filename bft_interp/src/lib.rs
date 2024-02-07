//! Brainfuck program interpreter
//!
//! Creates a virtual machine with a memory tape of cells of type T, and can accept and (eventually)
//! run a program

use std::num::NonZeroUsize;

use bft_types::BfProgram;

/// Represents a machine with a memory tape of cells. Accepts a type T for the tape
#[derive(Debug)]
pub struct VirtualMachine<T> {
    cells: Vec<T>,
    head: usize,
    tape_can_grow: bool,
}

impl<T> VirtualMachine<T>
where
    T: Clone + Default,
{
    /// Create a new VirtualMachine. Defaults to 30000 cells of memory if tape_size is zero.
    pub fn new(tape_size: Option<NonZeroUsize>, tape_can_grow: bool) -> Self {
        let tape_size = tape_size.map(NonZeroUsize::get).unwrap_or(30_000);

        Self {
            cells: vec![T::default(); tape_size],
            head: 0,
            tape_can_grow,
        }
    }

    /// Print out the intermediate representation of the program
    pub fn print_program(self, program: &BfProgram) {
        for instruction in program.instructions().iter() {
            println!("[{}] {}", program.name().display(), instruction)
        }
    }
}
