use core::fmt;

use eh1 as embedded_hal;
use embedded_hal::digital::ErrorKind::{self, Other};

/// Errors that may occur during mocking.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MockError {
    /// An unspecified error occurred.
    Other(String),
}

impl embedded_hal::digital::Error for MockError {
    fn kind(&self) -> ErrorKind {
        Other
    }
}

impl fmt::Display for MockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MockError::Other(msg) => write!(f, "Error: {:?}", msg),
        }
    }
}
