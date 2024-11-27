use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    InvalidUrl,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidUrl => write!(f, "Invalid URL"),
        }
    }
}
