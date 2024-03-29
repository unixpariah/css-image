use std::{error::Error, fmt};

#[derive(Debug)]
pub enum CssError {
    SizeError(&'static str),
    ContentError(&'static str),
}

impl fmt::Display for CssError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CssError::SizeError(message) => write!(f, "SizeError: {message}"),
            CssError::ContentError(message) => write!(f, "ContentError: {message}"),
        }
    }
}

impl Error for CssError {}

impl From<std::num::ParseIntError> for CssError {
    fn from(_: std::num::ParseIntError) -> Self {
        CssError::SizeError("SizeError: ParseIntError")
    }
}

impl From<&str> for CssError {
    fn from(_: &str) -> Self {
        CssError::ContentError("SizeError: &str")
    }
}
