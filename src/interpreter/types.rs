#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    StrLiteral(&'a str),
    Variable(&'a str),
    Operation(Operation),
    StartBracket,
    EndBracket
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operation {
    Plus,
    Minus,
    Mul,
    TernaryQuestion,
    TernaryElse
}

pub fn get_operation_priority(operation: Operation) -> u8 {
    match operation {
        Operation::Plus => 2,
        Operation::Minus => 2,
        Operation::Mul => 1,
        _ => unimplemented!("priority of non-math operation")
    }
}