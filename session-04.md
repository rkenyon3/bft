title: Rust Workshop - Session 4
class: animation-fade
layout: true

.bottom-bar[
{{title}}
]

---

count: false

# Leader slide

- Press 'b' to toggle 'blackout mode'
- Press 'p' to toggle 'presenter mode'
- Press 'c' to create a clone of this window which will keep up
- Press left/right to move around slides
- The next slide is where the presentation begins

???

This is where presenter notes appear

---

class: impact

# {{title}}

## Daniel Silverstone <br /><tt>&lt;dsilvers@digital-scurf.org&gt;</tt>

???

- Welcome everyone
- Explain the purpose of the session
  - Learn about more ways to organise code
  - Look at a couple of crates

---

title:
class: middle

# Topics

???

1. Modules - what, how, where, why?
2. Command line handling with `clap`
3. Command line handling with `clap -F derive`

---

title: Modules

- Why?

???

- We've seen crates as a way to separate out code
- But what if you have a huge amount of code to hold in one crate?
- Then we need modules

---

title: Modules - the filesystem

```
\ src
    + lib.rs
    + foo.rs
    \ bar/mod.rs
```

???

Any submodule exists in one of two files, either `foo.rs` for `mod foo` or
else `foo/mod.rs`.

---

title: Modules - in code

```rust
mod foo;

mod bar {
    // Some code
}
```

???

If you want to have a module, you have to tell the compiler where the code can
be found for the module. This is done with the `mod` syntax.

You can either say `mod foo;` and put the content in an external file like the
previous slide showed, or you can say `mod bar { ... }` and put the code inside
the module file you already have, like the test code we've done in the past
couple of weeks.

---

title: Modules - Conditional compilation

```rust
#[cfg(test)]
mod stuff {
	... // mock implementation
}

#[cfg(all(not(test), target_os = "windows"))]
mod stuff {
    ... // windows impl (maybe calls win32?)
}

#[cfg(all(not(test), not(target_os = "windows"))]
mod stuff {
    ... // unixy impl (maybe calls posix?)
}

// And bring it into scope
pub use self::stuff::*;
```

???

You can put conditions on module statements to allow them to be included
only in certain circumstances. We've seen this with `#[cfg(test)]` but it can
also be used to bring in different implementations (for example Windows vs
Linux impls of stuff)

---

title: Modules - Conditional compilation with external files

```rust
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(unix)] {
        mod unix;
        pub use self::unix::*;
    } else if #[cfg(windows)] {
        mod windows;
        pub use self::windows::*;
    } else {
        compile_error!("Unable to find implementation")
    }
}
```

???

You can also use external module files, though that gets a smidge more
complicated since we have to conditionalise the import too

---

title: Modules - Scopes

- Scopes

???

As you know, Rust is, for the most part, lexically scoped, though scopes
can look forward as well as backward (i.e. you don't have to pre-define like
in C/C++)

Modules introduce another level of scoping though, related to the visibility
modifier `pub`

---

title: Modules - Scopes

- Scopes
- Visibility

???

By default, things can only see public stuff in scopes which are not in the
direct ancestry path of themselves. Consider...

---

title: Modules - Scopes

- Scopes
- Visibility

```rust
struct Upper {}

mod One {
    struct Lower {}

    mod Two {

        fn onetwo() {}
    }
    fn threefour() {}
}
```

???

In this example, nothing is marked public, yet:

- `onetwo()` can see all of `Upper`, `Lower`, `threefour` with judicious
  use of `super::`
- `threefour()` can see `Upper` and `Lower`, but not `onetwo()`.

If we adjust things like this:

---

title: Modules - Scopes

- Scopes
- Visibility

```rust
struct Upper {}

mod One {
    struct Lower {}
*   #[cfg(test)]
    mod Two {
*       #[test]
        fn onetwo() {}
    }
    fn threefour() {}
}
```

???

…then we can perhaps see why test modules can access their parents
but the main code can't see the test functions.

---

# Some "special" visibilities

```rust
pub(crate) fn thingy() {}

mod stuff {
    pub(super) fn another() {}
    mod banana {
        pub(in crate::stuff) fn cheese() {}
    }
}
```

???

There are a number of additional visibility markers such as
`pub(crate)` you won't need them for now, but if you see this
syntactic construction you at least won't be confused.

We'll come back to visibility in a later session, but this should be
enough to help you with any odd errors you get while trying to write
and test your homework.

---

title: Moving on
class: impact

# Moving on

???

Now let's look at some libraries for doing something useful. We'll need one of
these, or something similar, to complete the homework this week…

---

title: Commandline handling - Clap

- Proper command line parsing is important
- We've been doing `std::env::args().skip(1).next()` and other horrors
- Let's look at a nicer way to do this

???

Clap is a good command line handler which is designed to produce nice command
line APIs with good help etc for minimum effort.

Clap uses something often referred to as "the builder pattern", so let's have
a look at that kind of thing…

---

title: Commandline handling - Clap - Simple example

```rust
Command::new("Brainfuck Tool")
     .version(env!("CARGO_PKG_VERSION"))
     .author("Daniel Silverstone")
     .about("Interprets Brainfuck programs")
     .arg(Arg::new("PROGRAM")
          .help("The program to interpret")
          .required(true))
     .get_matches()
```

???

Here's a very simple Clap setup which is for our `bft` program. It returns a
`matches` object.

---

title: Commandline handling - Clap - Using the matches

```rust
matches.get_one::<String>("PROGRAM").unwrap() // Safe because .required(true)

matches.try_get_one::<String>("PROGRAM")           // Nicer,
       .expect("BUG: mandatory PROGRAM not found") // but unnecessary
```

???

While this looks like more code than we had before (and it is) this is better
than using `env::args()` because we can more cleanly extend it going forward.

---

title: Clap in action
class: impact

# Clap in action

???

Let's create a new binary crate and have a go with clap.

- add clap
- use builder API, explore on docs.rs a bit
- goal is: `cargo run -- --help` and `cargo run -- filename`

---

title: Commandline handling - clap -F derive - Equivalence

```rust
/// A Brainfuck tool
#[derive(Parser, Debug)]
#[clap(author, version, about, name = "bft")]
struct Options {
    /// The program to interpret
    #[clap(name="PROGRAM")]
    program: PathBuf,
}

fn main() {
    let opt = Options::parse();
    println!("{:#?}", opt);
}
```

???

The `clap` crate's `derive` feature uses `clap` under the hood, but allows you to
define your options by means of a structure instead of open-coding them.

I'm not going to state which approach you should use, not least because I use both
in different circumstances. If you don't like the look of `clap` there are plenty of
others, but the material in the homework will assume you choose `clap`.

---

title: Homework

Play with `clap`'s `derive` feature, and maybe `clap` directly, because you'll
need one of them (or something
similar) for your homework. You'll also want to revise `PathBuf` and `Path`,
and you might want to learn about `OsStr` and `OsString` because they might
be important depending on how you do your homework.

Research bracket matching algorithms because, like the above, you'll need one.

If you run out of things to do, then research `std::io::Cursor` because we'll
be using it next time.

- The tasks will be emailed to you like last time.

---

count: false
class: impact

# Any questions?

???

Obvious any-questionsness
