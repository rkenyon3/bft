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

impl Instruction{
    pub fn from_char(c: char)->Option<Instruction>{
        match c {
            '<' => Some(Instruction::MoveLeft),
            '>' => Some(Instruction::MoveRight),
            '+' => Some(Instruction::Increment),
            '-' => Some(Instruction::Decrement),
            '.' => Some(Instruction::Input),
            ',' => Some(Instruction::Output),
            '[' => Some(Instruction::ConditionalJumpForward),
            ']' => Some(Instruction::ConditionalJumpBackward),
            _ => None
        }
    }
}


#[derive(Debug)]
struct BfProgram{
    name: String,
    instructions: Vec<Instruction>
}

impl BfProgram{
    fn from_file(file_path: &Path) -> std::io::Result<BfProgram>{
        
    }
}

