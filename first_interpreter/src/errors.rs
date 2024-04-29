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
    line: u32,
    loc: String,
    message: String,
    error_kind: ErrorKind,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[line {}] Error{:?}: {:?}",
            self.line, self.loc, self.message
        )
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new(line: u32, loc: &str, message: &str, error_kind: ErrorKind) -> Self {
        Self {
            line,
            loc: loc.to_string(),
            message: message.to_string(),
            error_kind,
        }
    }

    /// This logs a syntax error on a line with a given message
    pub fn report(line: u32, message: &str) {
        let err = Error::new(line, "", message, ErrorKind::SyntaxError);
        eprintln!("{}", err);
    }
}
