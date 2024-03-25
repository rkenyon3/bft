## General:
- VirtualMachine::new doesn't take BfProgram by borrow.
- in move_head_left I would maybe use self.head.checked_sub(1) but this isn't super important the code handles the case for zero correctly.
- in your impl for VirtualMachine you have T: Clone + Default + CellKind, but CellKind is already a sub-trait
  of Clone + Default, this means that specifying Clone + Default + CellKind is the same as specifying just CellKind.
  So the Additional bounds are not needed here.
- in your impl CellKind for u8 you have manually implemented the wrapping behaviour, the standard library already has functions for this.
  u8::wrapping_add/sub, you should use these instead.
- So the way you have written print_value technically isn't correct, .write can return Ok(0), indicating that the write
  didn't fail but it wrote zero bytes. This will never happen in your testing but you can handle it very easily by
  just using .write_all() instead of just .write().
- Your read/write error structs are overcomplicated, I might go over this in the session if we have time if not I will
  explain in our
- Some items are missing examples but overall the docs are pretty excellent :D

### Technicallyyyyy (doesn't really matter but might be interesting):
- In your read_value you return Err(VmError::from(...)), as you have implemented From<(...)> for your VmError,
  you can technically write this as Err((...))? as the ? operator calls .into on its argument and thus it can
  construct Result<(), VmError> from Result<(), (...)>.into(), but I would absolutely write it the way you have
  this is just for information.

### Ouch bugs:
- move_head_right is not correct, self.cells.capacity does not tell you how many cells there are.
  It tells you how many cells you can allocate *before* needing to reallocate.
  To correct this you must use Vec::len() instead.
  This is now safe but doesn't have the behaviour you expect as will still be added capacity that you are not using.
  To force the vector to grow and use all its space you can use the pattern of reserve then resize.
  So:
```rust
let mut v: Vec<i32> = vec![1, 2, 3];
// I want 10 elements
v.reserve(10);
// reserve guarantees us *at least* what we ask for
assert(v.capacity() >= 10);
// there are still no elements though
assert_eq(v.len(), 3);
// fill the vector with default elements
v.resize(v.capacity(), Default::default());
assert_eq(v.len(), v.capacity());
// this is now safe
assert_eq!(v[9], i32::default());
```

