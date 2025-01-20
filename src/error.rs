use std::fmt;
use std::io::Error as IoError;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Error {
    Io(Arc<IoError>),
    RootTypeNotStruct,
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
        Error::Io(Arc::new(IoError::new(
            std::io::ErrorKind::Other,
            format!("{}", msg),
        )))
    }
}
