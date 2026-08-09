use std::fmt;

use nom::IResult;
use nom::error::{ErrorKind, FromExternalError, ParseError};

pub type Input<'a> = &'a str;
pub type Result<'a, V> = IResult<Input<'a>, V, Error<'a>>;

/// Shorthand for the fully-specified [`nom::Parser`] this crate uses everywhere.
///
/// nom 8 combinators return opaque `impl nom::Parser` values rather than closures, so this is a
/// subtrait of [`nom::Parser`] rather than of `FnMut`. Plain functions and closures still qualify,
/// via nom's own blanket impl for `FnMut(I) -> IResult<I, O, E>`.
pub trait Parser<'a, V>: nom::Parser<Input<'a>, Output = V, Error = Error<'a>> {}

impl<'a, V, P> Parser<'a, V> for P where P: nom::Parser<Input<'a>, Output = V, Error = Error<'a>> {}

#[derive(Debug, PartialEq, Clone)]
pub struct ErrorDetail<'a> {
    pub input: &'a str,
    pub message: String,
}

impl<'a> ErrorDetail<'a> {
    pub fn new(input: &'a str, message: impl Into<String>) -> Self {
        let input = &input[..input.len().min(1)];
        Self {
            input,
            message: message.into(),
        }
    }
}

impl fmt::Display for ErrorDetail<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Replacement to [`nom::error::Error`].
#[derive(Debug, PartialEq)]
pub struct Error<'a> {
    pub input: Input<'a>,
    pub code: ErrorKind,
    pub detail: Option<ErrorDetail<'a>>,
}

impl<'a> Error<'a> {
    pub fn new(input: &'a str, code: ErrorKind, detail: Option<ErrorDetail<'a>>) -> Self {
        Error {
            input,
            code,
            detail,
        }
    }

    pub fn with_detail(&self, detail: ErrorDetail<'a>) -> Self {
        Error {
            input: self.input,
            code: self.code,
            detail: Some(detail),
        }
    }
}

/// Mandatory [`ParseError`] implementation.
impl<'a> ParseError<Input<'a>> for Error<'a> {
    fn from_error_kind(input: Input<'a>, kind: ErrorKind) -> Self {
        Error {
            input,
            code: kind,
            detail: None,
        }
    }

    fn append(_: Input<'a>, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a, E> FromExternalError<Input<'a>, E> for Error<'a> {
    fn from_external_error(input: Input<'a>, kind: ErrorKind, _e: E) -> Self {
        Error {
            input,
            code: kind,
            detail: None,
        }
    }
}
