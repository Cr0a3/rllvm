use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectError {
    DeclWithoutSymbol,
    UnknownFunction(String),
    UnknownTargetSymbol(String),
}

impl fmt::Display for ObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            ObjectError::DeclWithoutSymbol => "got decleration without symbol".to_string(),
            ObjectError::UnknownFunction(n) => format!("unknown function {}", n),
            ObjectError::UnknownTargetSymbol(n) => format!("unknown target symbol {}", n),
        };

        write!(f, "{}", str)
    }
}

impl std::error::Error for ObjectError {}