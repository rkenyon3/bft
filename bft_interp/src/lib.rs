//! Brainfuck program interpreter
//!
//! Creates a virtual machine with a memory tape of cells of type T, and can accept and (eventually)
//! run a program

use bft_types::BfProgram;

/// Represents a machine with a memory tape of cells. Accepts a type T for the tape
#[derive(Debug)]
pub struct VirtualMachine<T> {
    cells: Vec<T>,
    pointer: usize,
    tape_can_grow: bool,
}

impl<T> VirtualMachine<T> {
    /// Create a new VirtualMachine. Defaults to 30000 cells of memory if tape_size is zero.
    pub fn new(mut tape_size: usize, tape_can_grow: bool) -> Self {
        if tape_size == 0 {
            tape_size = 30000;
        }

        Self {
            cells: Vec::<T>::with_capacity(tape_size),
            pointer: 0,
            tape_can_grow,
        }
    }

    /// Get the value of the memory cell currently pointed at on the tape
    pub fn cell_value(&self, address: usize) -> Result<&T, Box<dyn std::error::Error>> {
        // TODO: check that this is safe
        Ok(&self.cells[address])
    }

    /// Add more cells to the tape. Attempts to add at least cell_count_to_add cells
    pub fn grow_tape(mut self, cell_count_to_add: usize) {
        self.cells.reserve(cell_count_to_add);
    }

    /// Print out the intermediate representation of the program
    pub fn print_program(self, program: &BfProgram) {
        for instruction in program.instructions().iter() {
            println!("[{}] {}", program.name().to_string_lossy(), instruction)
        }
    }
}
