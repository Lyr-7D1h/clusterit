use core::fmt;
use std::error;

#[derive(Debug)]
pub enum ConnectionError {
    ParseError(&'static str),
    Ssh(ssh::Error),
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::ParseError(s) => write!(f, "Connection Parsing Error: {s}"),
            &ConnectionError::Ssh(s) => s.fmt(f),
        }
    }
}

impl error::Error for ConnectionError {}

impl From<ssh::Error> for ConnectionError {
    fn from(e: ssh::Error) -> Self {
        ConnectionError::Ssh(e)
    }
}
