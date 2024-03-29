Session 6 homework
==================

As with session 5, this session's homework is entirely focussed on your `bft`
project.  Like in session 5, a lot of the detail will be left up to you.  At
the end of this session's homework, your `bft` tool will finally be able to be
interpreting the hello-world program which we have been carrying around since
session 1.

This is a large amount of work and you are not expected to necessarily complete
it within an hour.  Indeed I expect this may take some of you a number of days
of short efforts.

If you get stuck, gather questions and ask your peers or myself.  I'm happy
to run some recap workshops too.

Task One - Looping constructs
=============================

The looping constructs are the hardest bit for our interpreter to implement.
Both do a test and optional jump.

From Wikipedia, rephrased slightly:

`[`: if the cell at the head is zero, then, instead of moving the instruction
     pointer forward to the next command, jump it forward to the command after
     the matching `]` command.
`]`: if the cell at the head is nonzero, then instead of moving the
     instruction pointer forward to the next command, jump it back to the
     command after the matching `[` command.

We can simplify this by picking one or other of the brackets and implementing
the other as an unconditional jump to the matching bracket to allow the test
to be implemented at that bracket.

In order to do this, we clearly need to refine our programming pattern for the
instruction implementations.  Let's change `Result<(), VMError>` into
`Result<usize, VMError>` to return the *next* instruction pointer to use.  For
all the instructions we've already implemented, this will simply be the current
instruction pointer (program counter) plus one.

Once you've made the above changes to your VM struct and its documentation and
tests, we can begin the looping construct work.  If you didn't do the extra
credit a couple of weeks ago, now's the time to go back and do so, because it'll
really help you with the _matching_ point above.

Pick one of start-loop or end-loop to implement as the trivial case, implement
it, document it, and write tests for it.  It really doesn't matter which you pick
because all the work need happen only in one end of the looping construct.

Now, add a function for the other of the two, add the zero-cell check, and return
the appropriate new instruction pointer according to the check's result.
Document and test this function.

Task Two - A first VM loop
==========================

We're finally ready to add our initial VM loop.  We're going to be replacing
our debug printout interpreter from a while ago.  Our VM loop needs to take a
reader, and a writer, (probably separately) and will run the program until
either an error occurs, or else the program counter goes outside of the program
at which point the program terminates cleanly.  As such, the interpret method
like looks something like:

    pub fn interpret<R, W>(&mut self, input: R, output: W) -> Result<(), VMError>
      where
      R: Read,
      W: Write,

I imagine that the body of this function will consist of either a `loop {}` or
a `while {}` construct, with a `match` statement across the instruction at the
current program counter, dispatching to the requisite method implemented above,
updating the program counter, propagating errors as appropriate.

Don't forget to document and to test your VM loop as well.  Either by means
of IO, or, for now, by means of running a simple program and then checking the
content of one or more of the cells in the tape.

When you add support to your `main()` to call this interpreter loop, remember
that you will have to report errors nicely to the user, as returned from the
interpreter.

Assuming all has gone well, you ought to be able to, at this point, run `cargo
run -- helloworld.bf` where `helloworld.bf` is the BF program you received in
earlier homework.

Task Three - Make it neat
=========================

You may have noticed that the `helloworld.bf` I provided does not terminate
its output with a nice newline.  It would be much better if you could detect
that and output a newline (perhaps in `main()` perhaps in `VM::interpret()`)
should this happen, just so that output is neater.

For the full benefit of practice, make the implementation of this feature into
a struct which is parameterised over any `Write` implementation, proxies the
writes and flushes to an inner value and keeps track of whether the last thing
written was a newline.  Then on `Drop` it should write out the extra newline if
needed.  If you do this, remember writers take byte slices, not strings, so you
will have to work out how to represent a newline as a u8 rather than a char.

Don't forget to document it and test your solution, whichever way you choose.

