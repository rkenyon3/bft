#[derive(Debug)]
pub struct VirtualMachine<T> {
    cells: Vec<T>,
    pointer: usize,
    tape_can_grow: bool,
}

impl<T> VirtualMachine<T> {
    // TODO: look into taking Option<NonZeroUsize> rather than usize here
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

    pub fn cell_value(&self, address: usize) -> Result<&T, Box<dyn std::error::Error>>{
        // TODO: check that this is safe
        Ok(&self.cells[address])
    }

    pub fn grow_tape(mut self, cell_count_to_add: usize) {
        self.cells.reserve(cell_count_to_add);
    }
}
