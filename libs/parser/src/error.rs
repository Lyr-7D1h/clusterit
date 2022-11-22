use std::{error, fmt::Display, io};

#[derive(Debug)]
pub enum Error {
    ParseError { line: u32, message: String },
    IoError(io::Error),
}

impl Error {
    pub fn parse_error(line: u32, message: String) -> Error {
        Error::ParseError { line, message }
    }

    pub fn io_error<E: Into<Box<dyn error::Error + Send + Sync>>>(
        kind: io::ErrorKind,
        error: E,
    ) -> Error {
        Error::IoError(io::Error::new(kind, error))
    }
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError { line, message } => {
                write!(f, "[Line {}]: {}", line, message)
            }
            Error::IoError(e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}
