#[derive(Debug)]
pub struct Error {
    pub line: usize,
    pub message: String,
}


impl Error {
    pub fn new(line: usize, message: &str) -> Self {
        Self {
            line,
            message: message.to_string(),
        }
    }
}
