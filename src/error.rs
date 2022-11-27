use core::fmt;
use std::error;

use super::connection::ConnectionError;

#[derive(Debug)]
pub enum ClusteritErrorKind {
    Generic,
    ConnectionError,
    ParseError,
}

#[derive(Debug)]
pub struct ClusteritError {
    kind: ClusteritErrorKind,
    error: Box<dyn error::Error + Send + Sync>,
}

impl ClusteritError {
    pub fn new<E: Into<Box<dyn error::Error + Send + Sync>>>(
        kind: ClusteritErrorKind,
        error: E,
    ) -> ClusteritError {
        ClusteritError {
            kind,
            error: error.into(),
        }
    }
}

impl fmt::Display for ClusteritError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ClusteritErrorKind::ParseError => {
                write!(f, "Failed to parse: {}", self.error.to_string())
            }
            ClusteritErrorKind::ConnectionError => self.error.fmt(f),
            ClusteritErrorKind::Generic => {
                write!(f, "Something went wrong: {}", self.error.to_string())
            }
        }
    }
}

impl From<parser::Error> for ClusteritError {
    fn from(e: parser::Error) -> Self {
        ClusteritError::new(ClusteritErrorKind::ParseError, Box::new(e))
    }
}

impl error::Error for ClusteritError {}

impl From<ConnectionError> for ClusteritError {
    fn from(e: ConnectionError) -> Self {
        ClusteritError::new(ClusteritErrorKind::ConnectionError, Box::new(e))
    }
}

impl From<serde_json::Error> for ClusteritError {
    fn from(e: serde_json::Error) -> Self {
        ClusteritError::new(ClusteritErrorKind::ParseError, Box::new(e))
    }
}
