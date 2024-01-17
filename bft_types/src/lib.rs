use std::ffi::OsStr;
use std::fs;
use std::path::Path;

#[derive(Debug)]
enum Instruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Input,
    Output,
    ConditionalJumpForward,
    ConditionalJumpBackward,
}

impl Instruction {
    pub fn from_char(c: char) -> Option<Instruction> {
        match c {
            '<' => Some(Instruction::MoveLeft),
            '>' => Some(Instruction::MoveRight),
            '+' => Some(Instruction::Increment),
            '-' => Some(Instruction::Decrement),
            '.' => Some(Instruction::Input),
            ',' => Some(Instruction::Output),
            '[' => Some(Instruction::ConditionalJumpForward),
            ']' => Some(Instruction::ConditionalJumpBackward),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct BfProgram {
    name: Box<OsStr>,
    instructions: Vec<Instruction>,
}

impl BfProgram {
    // TODO: check file_path typing
    fn from_file(file_path: &Path) -> Result<BfProgram, Box<dyn std::error::Error>> {
        let file_name = file_path.file_name().ok_or("No filename provided")?;
        let file_contents = fs::read_to_string(file_path)?;
        Ok(Self::new(file_name, file_contents.as_str()))
    }

    /// Parse the str
    fn new(filename: &OsStr, file_contents: &str) -> Self {
        let mut instructions: Vec<Instruction> = Vec::new();

        // TODO: see if this can be shortened
        for c in file_contents.chars() {
            match Instruction::from_char(c) {
                None => (),
                Some(instr) => instructions.push(instr),
            }
        }

        Self {
            name: filename.into(),
            instructions,
        }
    }
}
