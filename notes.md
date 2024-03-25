# notes

- should be using PathBuf to store paths for CLI
- Path is read-only

struct Program{
    src: PathBuf,
    content: String
}

impl Program{
    fn new(src: impl AsRef<Path>, comtent: impl AsRef<str>) -> Self {
        let src - src.as_ref();
        let content = content.as_ref();
        Self {
            src:src.to_owned(),
            content: content.to_owned(),
        }
    }
}

this can also be written as in bft_interp lib.rs

# testing stuff

- crate called rstest might be useful

- look at rust testing book


# session 05 - traits, lifetimes, and borrows
- two sessions to go after this
- traits may specify required functions
- may depend on other traits, e.g.:
'pub trait MyThink: clone {
    fn make_fresh_copy(&self) -> Self{self.clone()}
}'
- read up on from<> and into<>
- can implement convert_from for any number of _non-overlapping_ types
- string::chars is a good example
- with impl in return type, this means that the caller only gets to know that the return type implements that trait
- can be useful if you can't name a return type
- in parameter list, impl in function args is short for where clause
- produces monomorphisation
- read about box dyn traits
- what is an instruction cache?
- look at standard library?
- fn my_func(a: &impl Thing) {} tends to produce faster code
- fn my_func(a: &dyn Thing) {} tends to produce smaller code

- **read up on std::io::cursor**
- read rust book chapter 10.2(?) traits
- **practice this**
- skim the advanced traits chapter
- read and *really* understand 10.3 validating references with lifetimes

# TODO - Session-04 review
- add more tests, including for unmatched '[' and ']'
- reword docs to be more helpful and dry

# Session 06
marker traits
- things with a sized marker trait have a known size at compile time
- send and sync are indicators for whether an object can safely traverse threads
- send objects can be moved safely to another thread
- sync objects can be safely shared across threads
- copy tells the compiler that the literal bits representing a value can be safely copied

# TODO
read up on drop, read, write, display and formatters, option, result, iterator

# session-05 review sam
- checked functions in move_head_left
- failing read/write can be caused with an empty buffer

# Session 07
- read up on coupling, coherence, and cohesion
- coupling is how much code in one place affects code in another. Tightly coupled code is spaghetti code
- coherence is how well isolated code is. Coherent, well-isolated code is easier to understand and use.
- refactoring is a specific term, and implies that the code being tested has a strong testing setup to confirm that the changes made no functional change. Writing new tests means you are not refactoring.
- 
- default visibility is "not public"
- accessible to stuff in the main.rs file and descendants
- pub (public) is widest possible visibility
- any code that can find a path to this item, can access it
- struct members are private by default
- trivial accessor methods are encouraged
- restricted public access levels exist
- pub(Crate)
- pub(super)
- pub(self)
- pub(in some::path)
- cargo semver checks