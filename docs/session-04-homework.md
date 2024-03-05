Session 4 homework
==================

As with session 3, this session's homework is entirely focussed on your `bft`
project.  However unlike session 3, this time a lot of the detail will be left
up to you.  At the end of this session's homework, your `bft` tool will have a
slightly nicer CLI arg parser and should be capable of syntax checking the
hello-world program which we have been carrying around since session 1.

Remember, from now on, homework review is up to you until the end of the project.
If you want me to review it, you need to ask me to.  Perhaps you can do this
by creating merge requests and asking me to review those - you don't have to do
one MR per task, nor one per homework session, but break things up logically.

Task One - The completionist
============================

If you've not completed everything from session 3's homework, including all
the optional effort which is not reliant on a specific CI system (i.e. all the
nice documentation, code arrangement, etc) then please go back and do so.

This means adding documentation to your `bft_interp` crate and also your `bft`
crate please.

Task Two - A CLI
================

Using Clap directly, Clap via the derive feature, or some other crate meant to
help you with CLIs, please add a CLI to your `bft` application. Your application
should require a positional argument called `PROGRAM` and report a suitable
error if it isn't supplied.

Your CLI should have the usual `--help` `--version` etc which anyone would
expect of a modern UNIXy program.

Handle your CLI in a separate module called `cli` which is in a separate file
`src/cli.rs`.

In addition, your program should accept `-c` or `--cells` with a non-zero
numeric argument to be the number of cells in the tape.  Also it should accept
a '-e' or '--extensible' which will turn on the auto-extending tape.  Note,
it should be an error for you to specify "--cells 0".

Process the command line arguments, report any issues, and prepare the
interpreter instance using them.  Validation of the command line arguments
should, ideally, be handled by the hooks of whichever CLI library you choose.

Task Three - Validating the program
===================================

Some arbitrary input cannot be guaranteed to be a syntactically valid BF
program. There are various ways in which a program could be invalid, but the one
we care about for this task is balanced square brackets. Since the behaviour of
the open square-bracket and close square-bracket require the location of the
matched bracket it's essential that they balance. However the angle brackets do
not have to be syntactically balanced.

Add a function to your program structure which will check that the
square-brackets are balanced, reporting the first problem you find.

While it won't be needed today, to get ahead for next time see if you can store
anything to make interpreting the program later more efficient. If you can work
something out, then store this useful data by making this analysis/checking
function take `&mut self` instead of just `&self`.

Ensure that this checking function is called in your `main()` and any errors are
reported.

Don't forget that this will need tests and documentation.

Task Four - Nicer exit
======================

Finally, to make things a little nicer for our users, we're going to stop
returning a `Result` from our `main()` and instead behave a little better.

Create a struct to represent your parsed CLI information if you do not already
have one from `clap`'s `derive` feature or the like.

Rename your current `main()` function something such as `run_bft()` and make
it take a struct which contains your CLI argument information all separated out.

Write a new `fn main() {}` which uses your CLI parser and then calls `run_bft()`
passing in the struct mentioned above, and if `run_bft()` returns an error,
format it nicely and then exit with a failure code of 1. You will need to find
out how to do this, it's possible with the standard library.

At this point, your code should be functionally equivalent to before, but
reporting errors more nicely. For bonus kudos, ensure any error messages mention
your program name, for example:

```
bft: Error in input file foo.bf, no close bracket found matching bracket at line 4 column 8
```

