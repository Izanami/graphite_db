use std::io;
use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
/// A list specifying general categories of Network error.
pub enum NetworkError {
    /// The error type for I/O operations of the Read, Write, Seek, and associated traits.
    Io(io::Error),
}

/// A specialized Result type for Network.
pub type Result<T> = result::Result<T, NetworkError>;

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NetworkError::Io(ref err) => write!(f, "IO error: {}", err),
        }
    }
}
impl error::Error for NetworkError {
    fn description(&self) -> &str {
        match *self {
            NetworkError::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            NetworkError::Io(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for NetworkError {
    fn from(err: io::Error) -> NetworkError {
        NetworkError::Io(err)
    }
}
