//! Instruction types for the BF interpreter to use.

use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

/// Types of Brainfuck instructions
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
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

impl Instruction {
    /// Parse a single char. If the character represents an instruction, return the corresponding type.
    pub fn from_char(c: char) -> Option<Instruction> {
        match c {
            '<' => Some(Instruction::MoveLeft),
            '>' => Some(Instruction::MoveRight),
            '+' => Some(Instruction::Increment),
            '-' => Some(Instruction::Decrement),
            '.' => Some(Instruction::Output),
            ',' => Some(Instruction::Input),
            '[' => Some(Instruction::ConditionalJumpForward),
            ']' => Some(Instruction::ConditionalJumpBackward),
            _ => None,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Instruction::MoveLeft => "MoveLeft",
            Instruction::MoveRight => "MoveRight",
            Instruction::Increment => "Increment",
            Instruction::Decrement => "Decrement",
            Instruction::Input => "Input",
            Instruction::Output => "Output",
            Instruction::ConditionalJumpForward => "ConditionalJumpForward",
            Instruction::ConditionalJumpBackward => "ConditionalJumpBackward",
        };

        write!(f, "{}", description)
    }
}

/// Representation of a Brainfuck instruction, including the instruction type, and the line number and column on which it appears
#[derive(Debug, PartialEq)]
pub struct LocalisedInstruction {
    /// The type of operation this instruction represents
    instruction: Instruction,
    /// The line number of the original file upon which this instruction appears
    line_num: usize,
    /// The column number of the original file in which this instruction appears
    column_num: usize,
}

impl LocalisedInstruction {
    /// Construct a new Instruction with a parsed instruction type
    pub fn new(instruction: Instruction, line_num: usize, column_num: usize) -> Self {
        Self {
            instruction,
            line_num,
            column_num,
        }
    }
}

impl Display for LocalisedInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}\t{}",
            self.line_num + 1,
            self.column_num + 1,
            self.instruction
        )
    }
}

/// Representation of a Brainfuck program, including it's name and a vector of Instructions
#[derive(Debug)]
pub struct BfProgram {
    /// Name of the file containing the original program
    name: PathBuf,
    /// A vector of instructions. Not sure how else to desribe it
    instructions: Vec<LocalisedInstruction>,
}

impl BfProgram {
    // TODO: check file_path typing
    /// Attempt to load a valid Brainfuck program from the specified file path
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> std::io::Result<BfProgram> {
        let file_contents = fs::read_to_string(&file_path)?;
        Ok(Self::new(file_path, file_contents.as_str()))
    }

    /// Construct a new BfProgram from a &str
    fn new<P: AsRef<Path>>(filename: P, file_contents: &str) -> Self {
        let mut instructions: Vec<LocalisedInstruction> = Vec::new();

        // TODO: see if this can be shortened
        for (line_number, file_line) in file_contents.lines().enumerate() {
            for (col_number, character) in file_line.chars().enumerate() {
                match Instruction::from_char(character) {
                    None => (),
                    Some(instr) => {
                        instructions.push(LocalisedInstruction::new(
                            instr,
                            line_number,
                            col_number,
                        ));
                    }
                }
            }
        }

        Self {
            name: filename.as_ref().to_path_buf(),
            instructions,
        }
    }

    /// Get the name of the program
    pub fn name(&self) -> &Path {
        &self.name
    }

    /// Borrow(?) a copy of the instructions
    pub fn instructions(&self) -> &[LocalisedInstruction] {
        &self.instructions
    }

    /// Analyse the program to ensure that it is syntactically valid
    pub fn analyse_program(&self) -> Result<(), String> {
        let mut bracket_count: usize = 0;
        // TODO: add functionality to store bracket locations here
        for program_instruction in self.instructions.iter() {
            if program_instruction.instruction == Instruction::ConditionalJumpForward {
                bracket_count += 1;
            } else if program_instruction.instruction == Instruction::ConditionalJumpBackward {
                bracket_count -= 1;
            }
        }

        if bracket_count == 0 {
            Ok(())
        } else {
            Err(String::from(
                "Program contains unbalanced conditional jumps ([])",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Check that each char that represents an instruction parses correctly
    #[test]
    fn parse_chars() {
        assert_eq!(Instruction::from_char('<'), Some(Instruction::MoveLeft));
        assert_eq!(Instruction::from_char('>'), Some(Instruction::MoveRight));
        assert_eq!(Instruction::from_char('+'), Some(Instruction::Increment));
        assert_eq!(Instruction::from_char('-'), Some(Instruction::Decrement));
        assert_eq!(Instruction::from_char(','), Some(Instruction::Input));
        assert_eq!(Instruction::from_char('.'), Some(Instruction::Output));
        assert_eq!(
            Instruction::from_char('['),
            Some(Instruction::ConditionalJumpForward)
        );
        assert_eq!(
            Instruction::from_char(']'),
            Some(Instruction::ConditionalJumpBackward)
        );
    }

    /// Check that a program can be constructed and records line and column numbers correctly
    #[test]
    fn parse_program() {
        let filename = Path::new("test_file.bf");
        let lines = "_<\n__<\n";
        let placeholder_instruction_type = Instruction::from_char('<').unwrap(); // probably shouldn't use unwrap here but I'm getting fed up of this and it'll do for now

        let bf_program = BfProgram::new(filename, lines);

        assert_eq!(bf_program.name(), filename);

        let expected_instruction =
            LocalisedInstruction::new(placeholder_instruction_type.clone(), 0, 1);
        assert_eq!(bf_program.instructions[0], expected_instruction);

        let expected_instruction =
            LocalisedInstruction::new(placeholder_instruction_type.clone(), 1, 2);
        assert_eq!(bf_program.instructions.get(1), Some(&expected_instruction));
    }
}
