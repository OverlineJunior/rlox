use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum ScanError {
    UnexpectedChar {
        ch: char,
        line: usize,
    },
    ExpectedDigitAfterDot {
        line: usize,
    },
    UnterminatedString {
        line: usize,
    },
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::UnexpectedChar { ch, line } => {
				write!(f, "[line {line}] Unexpected character `{}`", ch)
			}
            ScanError::ExpectedDigitAfterDot { line } => {
                write!(f, "[line {line}] Digit expected after dot")
            }
            ScanError::UnterminatedString { line } => {
                write!(f, "[line {line}] Unterminated string")
            }
        }
    }
}

impl fmt::Debug for ScanError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(self, f)
	}
}
