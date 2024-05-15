/// Represents an IO error
#[derive(Debug)]
struct IOError {
    message: String,
}

impl IOError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

/// Represents a syntax error
#[derive(Debug)]
struct SyntaxError {
    line: u32,
    message: String,
}

impl SyntaxError {
    pub fn new(line: u32, message: &str) -> SyntaxError {
        SyntaxError {
            line,
            message: message.to_string(),
        }
    }
}

/// Denotes what kinds of errors occurred
/// Non-exhaustive, other kinds might be added in the future
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Error used for syntax errors
    SyntaxError(SyntaxError),
    /// Error used for IO errors
    IOError(IOError),
    /// Error used for uncategorized errors
    GenericError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SyntaxError(err) => write!(f, "[line {}]: {:?}", err.line, err.message),
            Error::IOError(err) => write!(f, "{:?}", err.message),
            Error::GenericError(err_msg) => write!(f, "Error: {:?}", err_msg),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    /// This logs an error on a line with a given message
    fn report(&self) {
        eprintln!("{}", self);
    }

    /// This logs an [`Error::SyntaxError`] on a line with a given message
    pub fn report_syntax(line: u32, message: &str) -> Self {
        let err = Error::SyntaxError(SyntaxError::new(line, message));
        err.report();
        err
    }

    /// This logs an [`Error::IOError`] on a line with a given message
    pub fn report_io(message: &str) -> Self {
        let err = Error::IOError(IOError::new(message));
        err.report();
        err
    }

    /// This logs an [`Error::GenericError`] on a line with a given message
    pub fn report_generic(message: &str) -> Self {
        let err = Error::GenericError(message.to_string());
        err.report();
        err
    }
}
