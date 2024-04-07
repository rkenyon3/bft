//! Instruction types for the BF interpreter to use.

use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

/// Types of Brainfuck instructions
#[derive(Debug, PartialEq, Clone, Eq, Copy)]
pub enum Instruction {
    /// Increment the data pointer by one (to point to the next cell to the left).
    MoveLeft,
    /// Decrement the data pointer by one (to point to the next cell to the right).
    MoveRight,
    /// Increment the byte at the data pointer by one.
    Increment,
    /// Decrement the byte at the data pointer by one.
    Decrement,
    /// Accept one byte of input, storing its value in the byte at the data pointer.
    Input,
    /// Output the byte at the data pointer.
    Output,
    /// If the byte at the data pointer is zero, then instead of moving the instruction pointer
    /// forward to the next command, jump it forward to the command after the matching ] command.
    ConditionalJumpForward,
    /// If the byte at the data pointer is nonzero, then instead of moving the instruction pointer
    /// forward to the next command, jump it back to the command after the matching [ command.
    ConditionalJumpBackward,
}

impl Instruction {
    /// Parse a single char. If the character represents an instruction, return the corresponding
    /// type.
    ///
    /// ```
    ///# use bft_types::Instruction;
    ///  let c: char = '+';
    ///
    ///  let my_instruction = Instruction::from_char(c);
    /// ```
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
            Instruction::MoveLeft => "Move tape head left",
            Instruction::MoveRight => "Move tape head right",
            Instruction::Increment => "Increment the value in the cell under the head",
            Instruction::Decrement => "Decrement the value in the cell under the head",
            Instruction::Input => "Input a byte",
            Instruction::Output => "Output a byte",
            Instruction::ConditionalJumpForward => {
                "Jump forward to the matching ] if the cell is zero"
            }
            Instruction::ConditionalJumpBackward => {
                "Jump backwards to the matching [ if the cell is not zero"
            }
        };

        write!(f, "{}", description)
    }
}

/// A single program [Instruction] with the line and column number it originally appeared on. Line
/// and column numbers are 1-indexed
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LocalisedInstruction {
    /// The type of operation this instruction represents
    instruction: Instruction,
    /// The line number of the original file upon which this instruction appears, 1-indexed human-readable
    line_num: usize,
    /// The column number of the original file in which this instruction appears, 1-indexed human-readable
    column_num: usize,
}

impl LocalisedInstruction {
    /// Construct a new Instruction with a parsed instruction type
    /// ```
    ///# fn main() -> Result<(), Box<dyn std::error::Error>>{
    ///#    use bft_types;
    ///     let my_instruction = bft_types::Instruction::from_char('>').unwrap();
    ///     let line_number = 3;
    ///     let column_number = 7;
    ///
    ///     let my_localised_instruction = bft_types::LocalisedInstruction::new(
    ///         my_instruction, line_number, column_number
    ///     );
    ///#    Ok(())
    ///# }
    /// ```
    pub fn new(instruction: Instruction, line_num: usize, column_num: usize) -> Self {
        Self {
            instruction,
            line_num,
            column_num,
        }
    }

    /// Get the inner [Instruction]
    pub fn instruction(&self) -> Instruction {
        self.instruction
    }

    /// Get the human-readable (1-indexed) line number where this instruction appears in the
    /// original program file
    pub fn line_num(&self) -> usize {
        self.line_num
    }

    /// Get the human-readable (1-indexed) column number where this instruction appears in the
    /// original program file
    pub fn column_num(&self) -> usize {
        self.column_num
    }
}

impl Display for LocalisedInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}  {}",
            self.line_num, self.column_num, self.instruction
        )
    }
}

/// Representation of a Brainfuck program, including its name and a vector of [LocalisedInstruction]s
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BfProgram {
    /// Name of the file containing the original program
    name: PathBuf,
    /// A vector of instructions. Not sure how else to describe it
    instructions: Vec<LocalisedInstruction>,
    /// Vector to record, for each instruction, the index of the counterpart jump (if any)
    jump_map: Vec<Option<usize>>,
}

impl BfProgram {
    /// Attempt to load a valid Brainfuck program from the specified file path. Calls
    /// [BfProgram::new] internally.
    ///
    /// ```no_run
    ///# use bft_types::BfProgram;
    ///# use std::path::PathBuf;
    ///# fn main() -> Result<(), Box<dyn std::error::Error>>{
    ///  let bf_file = PathBuf::from("my_bf_program.bf");
    ///
    ///  let my_bf_program = BfProgram::from_file(bf_file)?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn from_file<P: AsRef<Path>>(
        file_path: P,
    ) -> Result<BfProgram, Box<dyn std::error::Error>> {
        let file_contents = fs::read_to_string(&file_path)?;
        Ok(Self::new(file_path, file_contents.as_str())?)
    }

    /// Construct a new [BfProgram] from a file path and a [str] that contains the program text.
    /// The program is analysed to compute a jump map and ensure that the program jumps ('[' and ']') are balanced.
    ///
    /// ```
    ///# use bft_types::BfProgram;
    ///# use std::path::PathBuf;
    ///# fn main() -> Result<(),Box<dyn std::error::Error>>{
    ///#
    ///  let bf_file = PathBuf::from("my_bf_program.bf");
    ///  let program_content = "[some bf code]++++.+++>[-],";
    ///
    ///  let my_bf_program: BfProgram = BfProgram::new(bf_file, program_content)?;
    ///#
    ///# Ok(())
    ///# }
    /// ```
    pub fn new<P: AsRef<Path>>(filename: P, file_contents: &str) -> Result<BfProgram, String> {
        let mut instructions: Vec<LocalisedInstruction> = Vec::new();
        let jump_map = Vec::new();

        for (line_number, file_line) in file_contents.lines().enumerate() {
            for (col_number, character) in file_line.chars().enumerate() {
                match Instruction::from_char(character) {
                    None => (),
                    Some(instr) => {
                        instructions.push(LocalisedInstruction::new(
                            instr,
                            line_number + 1,
                            col_number + 1,
                        ));
                    }
                }
            }
        }

        let mut new_program = Self {
            name: filename.as_ref().to_path_buf(),
            instructions,
            jump_map,
        };

        new_program.analyse_program()?;

        Ok(new_program)
    }

    /// Get the name of the program
    ///```
    ///# use bft_types::BfProgram;
    ///#
    ///# let my_bf_program = BfProgram::new("filename.bf","program text ++++.").unwrap();
    ///  let program_name = my_bf_program.name();
    ///```
    pub fn name(&self) -> &Path {
        &self.name
    }

    /// The [LocalisedInstruction]s that make up this program
    ///```
    ///# use bft_types::BfProgram;
    ///#
    ///# let my_bf_program = BfProgram::new("filename.bf","program text ++++.").unwrap();
    ///  let program_instructions = my_bf_program.localised_instructions();
    ///```  
    pub fn localised_instructions(&self) -> &[LocalisedInstruction] {
        &self.instructions
    }

    /// Given the index of an instruction in the program, get the index of the
    /// counterpart jump ('[' and ']')
    ///```
    ///# use bft_types::BfProgram;
    ///# fn main() -> Result<(),Box<dyn std::error::Error>>{
    ///  let my_bf_program = BfProgram::new("filename.bf","[program text] ++++.")?;
    ///  let current_program_address = 0;
    ///  let next_program_address = my_bf_program.jump_target(current_program_address);
    ///  assert_eq!(next_program_address, 2);
    ///# Ok(())
    ///# }
    ///```
    pub fn jump_target(&self, program_index: usize) -> usize {
        match &self.jump_map[program_index] {
            Some(target) => *target,
            None => 0,
        }
    }

    /// Analyse the program to ensure that it is syntactically valid, and record where the jumps map to.
    fn analyse_program(&mut self) -> Result<(), String> {
        let mut jump_instructions = Vec::<(usize, &LocalisedInstruction)>::new();

        for (program_index, program_instruction) in self.instructions.iter().enumerate() {
            // to begin with, store program_indexes and jump-forward instructuctions...
            if program_instruction.instruction == Instruction::ConditionalJumpForward {
                jump_instructions.push((program_index, program_instruction));
                self.jump_map.push(None); // push a placeholder
            }
            // ...and pop them back off their vector as we find their matches.
            // If we can't pop the corresponding [, we've got unmatched jumps
            else if program_instruction.instruction == Instruction::ConditionalJumpBackward {
                match jump_instructions.pop() {
                    Some(popped_jump) => {
                        let counterpart_index = popped_jump.0;
                        // add a new element pointing this jump back toward the next instruction after its counterpart ']'
                        self.jump_map.push(Some(counterpart_index + 1));
                        // and just update the existing entry for the initial '[' to point to the instruction after this one
                        self.jump_map[counterpart_index] = Some(program_index + 1);
                    }
                    None => {
                        return Err(format!(
                            "{}: Unmatched bracket on line {}, col {}",
                            self.name.to_string_lossy(),
                            program_instruction.line_num,
                            program_instruction.column_num
                        ))
                    }
                }
            } else {
                self.jump_map.push(None);
            }
        }

        match jump_instructions.pop() {
            Some(unmatched_jump) => Err(format!(
                "{}: Unmatched bracket on line {}, col {}",
                self.name.to_string_lossy(),
                unmatched_jump.1.line_num,
                unmatched_jump.1.column_num
            )),
            None => Ok(()),
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
        let lines = "..[<>]..[]..";
        let expected_jump_map: Vec<Option<usize>> = vec![
            None,
            None,
            Some(6),
            None,
            None,
            Some(3),
            None,
            None,
            Some(10),
            Some(9),
            None,
            None,
        ];

        let bf_program = BfProgram::new(filename, lines).unwrap();

        assert_eq!(bf_program.name(), filename);
        assert_eq!(bf_program.jump_map, expected_jump_map);
    }

    /// check that we find an unmatched [
    #[test]
    fn test_analyse_unmatched_open_square_bracket() {
        let filename = Path::new("test_file.bf");
        let lines = "_>>[<\n][[].,,<\n";

        let result = BfProgram::new(filename, lines);

        // Note: error message text matches the test program specifically
        let expected_result = Err(String::from(
            "test_file.bf: Unmatched bracket on line 2, col 2",
        ));

        assert_eq!(result, expected_result);
    }

    /// check that we find an unmatched ]
    #[test]
    fn test_analyse_unmatched_close_square_bracket() {
        let filename = Path::new("test_file.bf");
        let lines = "_>>[<\n]].,,<\n";

        let result = BfProgram::new(filename, lines);

        // Note: error message text matches the test program specifically
        let expected_result = Err(String::from(
            "test_file.bf: Unmatched bracket on line 2, col 2",
        ));

        assert_eq!(result, expected_result);
    }
}
