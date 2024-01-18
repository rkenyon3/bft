//! Instruction types for the BF interpreter to use.

use std::ffi::OsStr;
use std::fmt::Display;
use std::fs;
use std::path::Path;

/// Types of Brainfuck instructions (descriptions thieved from wikipedia)
#[derive(Debug, PartialEq, Clone)]
pub enum InstructionType {
    /// Increment the data pointer by one (to point to the next cell to the right).
    MoveLeft,
    /// Decrement the data pointer by one (to point to the next cell to the left). 
    MoveRight,
    /// Increment the byte at the data pointer by one.
    Increment,
    /// Decrement the byte at the data pointer by one.
    Decrement,
    /// Accept one byte of input, storing its value in the byte at the data pointer.
    Input,
    /// Output the byte at the data pointer.
    Output,
    /// If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
    ConditionalJumpForward,
    /// If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.
    ConditionalJumpBackward,
}

impl InstructionType {
    /// Parse a single char. If the character represents an instruction, return the corresponding type.
    pub fn from_char(c: char) -> Option<InstructionType> {
        match c {
            '<' => Some(InstructionType::MoveLeft),
            '>' => Some(InstructionType::MoveRight),
            '+' => Some(InstructionType::Increment),
            '-' => Some(InstructionType::Decrement),
            '.' => Some(InstructionType::Output),
            ',' => Some(InstructionType::Input),
            '[' => Some(InstructionType::ConditionalJumpForward),
            ']' => Some(InstructionType::ConditionalJumpBackward),
            _ => None,
        }
    }
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            InstructionType::MoveLeft => "MoveLeft",
            InstructionType::MoveRight => "MoveRight",
            InstructionType::Increment => "Increment",
            InstructionType::Decrement => "Decrement",
            InstructionType::Input => "Input",
            InstructionType::Output => "Output",
            InstructionType::ConditionalJumpForward => "ConditionalJumpForward",
            InstructionType::ConditionalJumpBackward => "ConditionalJumpBackward",
        };

        write!(f, "{}", description)
    }
}

/// Representation of a Brainfuck instruction, including the instruction type, and the line number and column on which it appears
#[derive(Debug, PartialEq)]
pub struct Instruction {
    /// The type of operation this instruction represents
    instruction_type: InstructionType,
    /// The line number of the original file upon which this instruction appears
    line_num: usize,
    /// The column number of the original file in which this instruction appears
    column_num: usize,
}

impl Instruction {
    /// Construct a new Instruction with a parsed instruction type
    pub fn new(instruction_type: InstructionType, line_num: usize, column_num: usize) -> Self {
        Self {
            instruction_type,
            line_num,
            column_num,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}\t{}",
            self.line_num + 1, self.column_num + 1, self.instruction_type
        )
    }
}


/// Representation of a Brainfuck program, including it's name and a vector of Instructions
#[derive(Debug)]
pub struct BfProgram {
    /// Name of the file containing the original program
    name: Box<OsStr>,
    /// A vector of instructions. Not sure how else to desribe it
    instructions: Vec<Instruction>,
}

impl BfProgram {
    // TODO: check file_path typing
    /// Attempt to load a valid Brainfuck program from the specified file path
    pub fn from_file(file_path: &Path) -> Result<BfProgram, Box<dyn std::error::Error>> {
        let file_name = file_path.file_name().ok_or("No filename provided")?;
        let file_contents = fs::read_to_string(file_path)?;
        Ok(Self::new(file_name, file_contents.as_str()))
    }

    /// Construct a new BfProgram from a &str
    fn new(filename: &OsStr, file_contents: &str) -> Self {
        let mut instructions: Vec<Instruction> = Vec::new();

        // TODO: see if this can be shortened
        for (line_number, file_line) in file_contents.lines().enumerate() {
            for (col_number, character) in file_line.chars().enumerate() {
                match InstructionType::from_char(character) {
                    None => (),
                    Some(instr) => {
                        instructions.push(Instruction::new(instr, line_number, col_number));
                    }
                }
            }
        }

        Self {
            name: filename.into(),
            instructions,
        }
    }

    /// Get the name of the program
    pub fn name(&self) -> &OsStr {
        &self.name
    }

    /// Borrow(?) a copy of the instructions
    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions[..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Check that each char that represents an instruction parses correctly
    #[test]
    fn parse_chars() {
        assert_eq!(
            InstructionType::from_char('<'),
            Some(InstructionType::MoveLeft)
        );
        assert_eq!(
            InstructionType::from_char('>'),
            Some(InstructionType::MoveRight)
        );
        assert_eq!(
            InstructionType::from_char('+'),
            Some(InstructionType::Increment)
        );
        assert_eq!(
            InstructionType::from_char('-'),
            Some(InstructionType::Decrement)
        );
        assert_eq!(
            InstructionType::from_char(','),
            Some(InstructionType::Input)
        );
        assert_eq!(
            InstructionType::from_char('.'),
            Some(InstructionType::Output)
        );
        assert_eq!(
            InstructionType::from_char('['),
            Some(InstructionType::ConditionalJumpForward)
        );
        assert_eq!(
            InstructionType::from_char(']'),
            Some(InstructionType::ConditionalJumpBackward)
        );
    }

    /// Check that a program can be constructed and records line and column numbers correctly
    #[test]
    fn parse_program() {
        let filename = OsStr::new("test_file.bf");
        let lines = "_<\n__<\n";
        let placeholder_instruction_type = InstructionType::from_char('<').unwrap(); // probably shouldn't use unwrap here but I'm getting fed up of this and it'll do for now

        let bf_program = BfProgram::new(filename, lines);

        assert_eq!(bf_program.name(), filename);

        let expected_instruction = Instruction::new(placeholder_instruction_type.clone(), 0, 1);
        assert_eq!(bf_program.instructions.get(0), Some(&expected_instruction));

        let expected_instruction = Instruction::new(placeholder_instruction_type.clone(), 1, 2);
        assert_eq!(bf_program.instructions.get(1), Some(&expected_instruction));
    }
}
