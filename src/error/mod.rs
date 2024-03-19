pub mod parse_error;
pub mod scan_error;

use std::fmt;

use parse_error::ParseError;
use scan_error::ScanError;

pub enum Error {
    Scan(ScanError),
    Parse(ParseError),
}

impl From<ScanError> for Error {
    fn from(err: ScanError) -> Self {
        Error::Scan(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::Parse(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Scan(err) => write!(f, "{err}"),
            Error::Parse(err) => write!(f, "{err}"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
