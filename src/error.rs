use std::fmt;
use std::error;

#[derive(Debug, Clone)]
pub enum ErrorType {
    Syntax,
}

#[derive(Debug, Clone)]
pub struct LoxError {
    _type: ErrorType,
    line: usize,
    message: String,
}

impl LoxError {
    fn new(_type: ErrorType, message: String, line: usize) -> Self {
        LoxError { _type, line, message }
    }

    pub fn syntax(line: usize, message: String) -> Self {
        LoxError::new(ErrorType::Syntax, message, line)
    }
}

pub type LoxResult<T> = Result<T, LoxError>;

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[line {}] {:?}Error: {}", self.line, self._type, self.message)
    }
}

impl error::Error for LoxError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
