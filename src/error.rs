use std::fmt;
use std::io::Error as IoError;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Error {
    Io(Arc<IoError>),
    RootTypeNotStruct,
    Unsupported(&'static str),
    Message(String),
    UnexpectedEof,
    ExpectedEquals,
    ExpectedNewline,
    ExpectedDot,
    InvalidBool,
    InvalidNumber,
    TrailingCharacters,
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::Io(Arc::new(error))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "{}", err),
            Error::RootTypeNotStruct => write!(f, "Root type must be a struct"),
            Error::Unsupported(msg) => write!(f, "Type `{}` is unsupported", msg),
            Error::Message(msg) => write!(f, "{}", msg),
            Error::UnexpectedEof => write!(f, "Unexpected end of file"),
            Error::ExpectedEquals => write!(f, "Expected '=' character"),
            Error::ExpectedNewline => write!(f, "Expected newline character"),
            Error::ExpectedDot => write!(f, "Expected '.' character"),
            Error::InvalidBool => write!(f, "Invalid boolean value"),
            Error::InvalidNumber => write!(f, "Invalid number"),
            Error::TrailingCharacters => write!(f, "Trailing characters after value"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        eprintln!("{}", msg);
        Error::Io(Arc::new(IoError::other(format!("{}", msg))))
    }
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Message(format!("{}", msg))
    }
}
