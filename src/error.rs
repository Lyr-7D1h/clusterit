use core::fmt;
use std::error;

use super::connection::ConnectionError;

#[derive(Debug)]
pub enum ClusteritError {
    ConnectionError(ConnectionError),
    ParseError(String),
}

impl fmt::Display for ClusteritError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClusteritError::ParseError(s) => write!(f, "Config Parsing Error: {s}"),
            ClusteritError::ConnectionError(e) => e.fmt(f),
        }
    }
}

impl error::Error for ClusteritError {}

impl From<ConnectionError> for ClusteritError {
    fn from(e: ConnectionError) -> Self {
        ClusteritError::ConnectionError(e)
    }
}

impl From<serde_json::Error> for ClusteritError {
    fn from(e: serde_json::Error) -> Self {
        ClusteritError::ParseError(e.to_string())
    }
}
