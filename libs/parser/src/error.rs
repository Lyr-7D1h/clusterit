use std::fmt::Display;

pub struct Error {
    line: u32,
    message: String,
}

impl Error {
    pub fn new() {}
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Line {self.line}]: {message}")
    }
}
