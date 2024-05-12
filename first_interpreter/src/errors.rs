/// Denotes what kinds of errors occurred
/// Non-exhaustive, other kinds might be added in the future
#[non_exhaustive]
#[derive(Debug)]
pub enum ErrorKind {
    SyntaxError,
    IOError,
}

/// Error represents a structure for error handling
#[derive(Debug)]
pub struct Error {
    line: Option<u32>,
    loc: Option<String>,
    message: String,
    error_kind: ErrorKind,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.line.is_some() && self.loc.is_some() {
            return write!(
                f,
                "[line {}] Error{:?}: {:?}",
                self.line.unwrap(),
                self.loc.clone().unwrap(),
                self.message
            );
        }

        if self.line.is_some() {
            return write!(f, "[line {}]: {:?}", self.line.unwrap(), self.message);
        }

        write!(f, "{:?}", self.message)
    }
}

impl std::error::Error for Error {}

impl Error {
    fn new(line: Option<u32>, loc: Option<&str>, message: &str, error_kind: ErrorKind) -> Self {
        Self {
            line,
            loc: loc.map(|l| l.to_string()),
            message: message.to_string(),
            error_kind,
        }
    }

    /// This logs an error on a line with a given message
    fn report(line: Option<u32>, message: &str, error_kind: ErrorKind) -> Self {
        let err = Error::new(line, None, message, error_kind);
        eprintln!("{}", err);
        err
    }

    /// This logs an [`ErrorKind::SyntaxError`] on a line with a given message
    pub fn report_syntax(line: Option<u32>, message: &str) -> Self {
        Error::report(line, message, ErrorKind::SyntaxError)
    }

    /// This logs a [`ErrorKind::IOError`] on a line with a given message
    pub fn report_io(line: Option<u32>, message: &str) -> Self {
        Error::report(line, message, ErrorKind::IOError)
    }
}
