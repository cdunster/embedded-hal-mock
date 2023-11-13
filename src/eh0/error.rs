use core::fmt;

/// Errors that may occur during mocking.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MockError {
    /// An unspecified error occurred.
    Other(String),
}

impl fmt::Display for MockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MockError::Other(msg) => write!(f, "Error: {:?}", msg),
        }
    }
}
