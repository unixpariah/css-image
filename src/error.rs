use std::{error::Error, fmt};

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum CssError<'a> {
    SizeError(&'a str),
    ContentError(&'a str),
    ParseError,
}

impl fmt::Display for CssError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CssError::SizeError(message) => write!(f, "SizeError: {message}"),
            CssError::ContentError(message) => write!(f, "ContentError: {message}"),
            CssError::ParseError => write!(f, "ParseError: Failed to parse CSS"),
        }
    }
}

impl Error for CssError<'_> {}

impl From<std::num::ParseFloatError> for CssError<'_> {
    fn from(_: std::num::ParseFloatError) -> Self {
        CssError::SizeError("SizeError: ParseFloatError")
    }
}

impl From<&str> for CssError<'_> {
    fn from(_: &str) -> Self {
        CssError::ContentError("SizeError: &str")
    }
}
