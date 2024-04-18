use std::{error::Error, fmt};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum CssError<'a> {
    SizeError(&'a str),
    ContentError(&'a str),
    ParseError,
    FontError(&'a str),
}

impl fmt::Display for CssError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CssError::SizeError(message) => write!(f, "SizeError: {message}"),
            CssError::ContentError(message) => write!(f, "ContentError: {message}"),
            CssError::ParseError => write!(f, "ParseError: Failed to parse CSS"),
            CssError::FontError(message) => write!(f, "FontError: {message}"),
        }
    }
}

impl Error for CssError<'_> {}

impl From<std::num::ParseFloatError> for CssError<'_> {
    fn from(_: std::num::ParseFloatError) -> Self {
        CssError::SizeError("ParseFloatError")
    }
}

impl From<std::num::ParseIntError> for CssError<'_> {
    fn from(_: std::num::ParseIntError) -> Self {
        CssError::SizeError("ParseIntError")
    }
}

impl From<&str> for CssError<'_> {
    fn from(_: &str) -> Self {
        CssError::ContentError("&str")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_error() {
        let error = CssError::SizeError("Test");
        assert_eq!(error.to_string(), "SizeError: Test");

        let error = CssError::ContentError("Test");
        assert_eq!(error.to_string(), "ContentError: Test");

        let error = CssError::ParseError;
        assert_eq!(error.to_string(), "ParseError: Failed to parse CSS");

        let error = CssError::FontError("Test");
        assert_eq!(error.to_string(), "FontError: Test");

        let error = CssError::SizeError("Test");
        assert_eq!(error.to_string(), "SizeError: Test");

        let error = CssError::SizeError("Test");
        assert_eq!(error.to_string(), "SizeError: Test");

        let error = "";
        let error = CssError::from(error);
        assert_eq!(error.to_string(), "ContentError: &str");

        let error = "".parse::<f32>().unwrap_err();
        let error = CssError::from(error);
        assert_eq!(error.to_string(), "SizeError: ParseFloatError");

        let error = "".parse::<i32>().unwrap_err();
        let error = CssError::from(error);
        assert_eq!(error.to_string(), "SizeError: ParseIntError");
    }
}
