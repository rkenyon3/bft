Session 5 homework
==================

As with session 4, this session's homework is entirely focussed on your `bft`
project.  Like in session 4, a lot of the detail will be left up to you.  At
the end of this session's homework, your `bft` tool will be closer to
interpreting the hello-world program which we have been carrying around since
session 1.

This is a large amount of work and you are not expected to necessarily complete
it within an hour.  You have a fortnight to get all of this done, but don't
leave it to the last minute, it won't be trivial to do.  Remember to ask questions
early, don't leave it until next Monday before trying to do the homework.

Task One - Shaking your head
============================

It's time - We're going to start to make our basic VM in `bft_interp` into a BF
interpreter.  To do this we're going to need to extend our struct slightly.

Firstly, you're going to want to add a member to the struct to represent the
head location on the tape.  This should default to the first tape location
(zero).  It's posible you already did this, in which case, well done, move on.

This member will need some careful handling, at this point our interpreter
could hit a runtime error (e.g. the head falling off either end of the tape).
So perhaps pause and give some thought to how you'll manage that eventuality.

Add an enumeration to your `bft_interp` crate which will be the errors returned
by the VM.  I suggest that the first variant should be for the head being moved
to an invalid position (before zero, or after the last cell in a non
auto-extending case).  It'd be helpful if the variant were constructed with the
marked instruction which caused the problem, so we can report the error nicely.

To make this possible to do, we're going to want to know the "program counter"
for the VM, so add this to your struct as well, and notice that we'll need the
program borrow in the struct as well.  Alter the `new` method to also take
a borrow of a program, adjust your tests appropriately, and also the `main`
function in `bft`.  You can remove your vestigial `interpret` function's
program argument and rewrite it to use the member in the struct.

Now you can add a "Move Head Left" and "Move Head Right" pair of methods to
your struct's impl block which do the requisite work on the head position
member (so they need to take `&mut self`) and return something like a
`Result<(), VMError>` where `VMError` is the enumeration mentioned above.

Remember to add documentation and tests for these functions, testing both the
success and the failure situations.

Task Two - Traits and maths
===========================

Add a trait to your `bft_interp` crate.  I suggest calling this `CellKind` or
some similar name.  Since cells must meet whatever constraints you wrote for
your `new()` method's `impl` block, make those constraints be required by
your new trait as well.

In order to add increment/decrement functions to your interpreter, we will need
to be able to increment/decrement values in the cells. Add methods to your trait
which represent in-place wrapping increment/decrement.

Next, swap out your constraint on your `impl` block for this new trait, and
following the same pattern as for the move-left and move-right methods, add
methods for incrementing and decrementing the values in the cells.

In order to be able to test all this, we will need an implementation of this new
trait. The obvious type to implement it for is `u8` since that's the default
type that Brainfuck VMs use. Write an `impl CellKind for u8`.

Now that you can, please remember to write some tests, docs, etc, so that it's
nice and clear what's going on.

Note:

It's possible that `num` or a similar crate will have something which can do this
for you, but for the purposes of this exercise I'm want you to design a trait
and implement it for u8 yourself.

Task Three - Basic I/O
======================

Now we want to add I/O.  We want to do this moderately generically in order
that we can test these functions usefully.  

So, let's write the `read_value()` method in our interpreter. This function will
be for reading a value from some `Read` implementation into the current cell. IO
in Brainfuck is always bytes, so you will probably need to write a `set_value()`
method into your `CellKind` trait, which takes a `u8` to set the cell to.

Note that since this will introduce IO errors into your interpreter, you should
add a variant to your `VMError` enumeration for IO errors. It should take the IO
error *and* the marked instruction.

While you can wrapper up the IO error manually, it'd be nice to enable the use
of the `?` operator on the `.read()` call by adding a From impl to convert from
a `std::io::Error` to your `VMError` type. Because of the presence of the marked
instruction, this might be a little hard, so see if you can find a way around
this.  If you cannot, then don't worry, just do whatever you can.

Next, write an interpreter function to output bytes. This function should take a
Write impl, and do similarly to the above, including sorting out a `CellKind`
method for retrieving a byte from a cell, and wrappering up IO errors
accordingly.

Remember that your new interpreter methods need documentation and also tests.
You can use the `std::io::Cursor<Vec<u8>>` type to provide readers and writers
for the purpose of testing your input and output functions.

Task Four - Thinking ahead
==========================

There are two BF instructions left which we've not implemented.  These will be
tackled in the next homework, but before we can usefully write these functions,
we will want to do some more stuff with our program struct.  Have a think
about what that might be, in preparation for the next homework.

Then, once you've forgotten what you were working on above, go back to your
codebase, render the docs out, and review them - are you happy with how they
look, could you write more docs, or better examples?  Really put some polish
into things now.
