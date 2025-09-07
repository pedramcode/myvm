use std::fmt::Display;

#[derive(Debug)]
/// Virtual machine errors
pub enum VMError {
    /// Stackoverflow error
    StackOverflow,
    /// Invalid size error
    InvalidSize(String),
    /// Empty container error
    EmptyContainer(String),
    /// Invalid size error
    InvalidAddress,
    /// Invalid register
    InvalidRegister,
    /// Invalid opcode
    InvalidOpcode,
    /// Invalid return call
    InvalidReturn,
    /// Invalid interrupt module
    InvalidModule,
    /// Invalid interrupt function
    InvalidFunction,
    /// Division by zero
    DivisionByZero,
}

impl Display for VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VMError::StackOverflow => write!(f, "Stackoverflow error"),
            VMError::InvalidSize(message) => write!(f, "Invalid size: {}", message),
            VMError::EmptyContainer(message) => write!(f, "Empty container: {}", message),
            VMError::InvalidAddress => write!(f, "Invalid address"),
            VMError::InvalidRegister => write!(f, "Invalid register"),
            VMError::InvalidOpcode => write!(f, "Invalid opcode"),
            VMError::InvalidReturn => write!(f, "Invalid return"),
            VMError::InvalidModule => write!(f, "Invalid interrupt module"),
            VMError::InvalidFunction => write!(f, "Invalid interrupt function"),
            VMError::DivisionByZero => write!(f, "Division by zero"),
        }
    }
}