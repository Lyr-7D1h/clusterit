use core::fmt;
use std::error;
use std::io;

#[derive(Debug)]
pub enum ConnectionError {
    ParseError(&'static str),
    Io(io::Error),
    Other(&'static str),
    Ssh(ssh2::Error),
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::ParseError(s) => write!(f, "Connection Parsing Error: {s}"),
            ConnectionError::Other(s) => write!(f, "Something went wrong: {s}"),
            ConnectionError::Ssh(s) => s.fmt(f),
            ConnectionError::Io(s) => s.fmt(f),
        }
    }
}

impl error::Error for ConnectionError {}

impl From<ssh2::Error> for ConnectionError {
    fn from(e: ssh2::Error) -> Self {
        ConnectionError::Ssh(e)
    }
}

impl From<io::Error> for ConnectionError {
    fn from(e: io::Error) -> Self {
        ConnectionError::Io(e)
    }
}
