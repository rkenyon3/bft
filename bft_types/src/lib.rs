use std::ffi::OsStr;
use std::fmt::Display;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Clone)]
pub enum InstructionType {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Input,
    Output,
    ConditionalJumpForward,
    ConditionalJumpBackward,
}

impl InstructionType {
    pub fn from_char(c: char) -> Option<InstructionType> {
        match c {
            '<' => Some(InstructionType::MoveLeft),
            '>' => Some(InstructionType::MoveRight),
            '+' => Some(InstructionType::Increment),
            '-' => Some(InstructionType::Decrement),
            '.' => Some(InstructionType::Input),
            ',' => Some(InstructionType::Output),
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

#[derive(Debug, PartialEq)]
pub struct Instruction {
    instruction_type: InstructionType,
    line_num: usize,
    column_num: usize,
}

impl Instruction {
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
            self.line_num, self.column_num, self.instruction_type
        )
    }
}

#[derive(Debug)]
pub struct BfProgram {
    // TODO: check members here are private
    name: Box<OsStr>,
    instructions: Vec<Instruction>,
}

impl BfProgram {
    // TODO: check file_path typing
    pub fn from_file(file_path: &Path) -> Result<BfProgram, Box<dyn std::error::Error>> {
        let file_name = file_path.file_name().ok_or("No filename provided")?;
        let file_contents = fs::read_to_string(file_path)?;
        Ok(Self::new(file_name, file_contents.as_str()))
    }

    /// Parse the str
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

    pub fn name(&self) -> &OsStr {
        &self.name
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions[..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            InstructionType::from_char('.'),
            Some(InstructionType::Input)
        );
        assert_eq!(
            InstructionType::from_char(','),
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
