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
- read up on from and into