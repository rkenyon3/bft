pub struct VirtualMachine<T> {
    cells: Vec<T>,
    pointer: usize,
    tape_size: usize,
    tape_can_grow: bool,
}

impl<T> VirtualMachine<T> {
    pub fn new(mut tape_size: usize, tape_can_grow: bool) -> Self {
        if tape_size == 0 {
            tape_size = 30000;
        }

        Self {
            cells: Vec::<T>::new(),
            pointer: 0,
            tape_size,
            tape_can_grow,
        }
    }
}
