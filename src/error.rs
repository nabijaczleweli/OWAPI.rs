use std::error::Error as StdError;
use std::fmt;

use reqwest;
use serde_json;


/// Generic error type colluding errors that may occur in the library.
#[derive(Debug)]
pub enum Error {
    /// A network request failed.
    Network(reqwest::Error),
    /// Parsing JSON failed.
    Parse(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Network(ref e) => fmt::Display::fmt(e, f),
            Error::Parse(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Network(ref e) => e.description(),
            Error::Parse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Network(ref e) => Some(e),
            Error::Parse(ref e) => Some(e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Network(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Parse(e)
    }
}
