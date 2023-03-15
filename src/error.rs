#[derive(Debug)]
pub struct ScanError {
    pub line: usize,
    pub message: String,
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub error_type: ErrorType,
}

impl Error {
    pub fn new(message: &str, error_type: ErrorType) -> Self {
        Self {
            message: message.to_string(),
            error_type,
        }
    }
}

#[derive(Debug)]
pub enum ErrorType {
    ScanError,
    SyntaxError,
    RuntimeError,
}


impl ScanError {
    pub fn new(line: usize, message: &str) -> Self {
        Self {
            line,
            message: message.to_string(),
        }
    }
}
