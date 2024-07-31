use crate::io;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt::{self, Debug, Display};
use core::result;
use core::str::FromStr;
use serde::{de, ser};
#[cfg(feature = "std")]
use std::error;
#[cfg(feature = "std")]
use std::io::ErrorKind;

pub type Result<T> = result::Result<T, Error>;

pub struct Error {
    err: Box<ErrorImpl>,
}

impl Error {
    pub fn line(&self) -> usize {
        self.err.line
    }

    pub fn column(&self) -> usize {
        self.err.column
    }

    pub fn classify(&self) -> Category {
        match self.err.code {
            ErrorCode::Message(_) => Category::Data,
            ErrorCode::Io(_) => Category::Io,
            ErrorCode::EofWhileParsingObject | ErrorCode::EofWhileParsingString => Category::Eof,
            ErrorCode::ExpectedSomeValue | ErrorCode::InvalidUnicodeCodePoint => Category::Syntax,
        }
    }

    /// Returns true if this error was caused by a failure to read or write bytes on an I/O stream.
    pub fn is_io(&self) -> bool {
        self.classify() == Category::Io
    }

    /// Returns true if this error was caused by input that was not syntactically valid VDF.
    pub fn is_syntax(&self) -> bool {
        self.classify() == Category::Syntax
    }

    /// Returns true if this error was caused by input data that was semantically incorrect.
    pub fn is_data(&self) -> bool {
        self.classify() == Category::Data
    }

    /// Returns true if this error was caused by prematurely reaching the end of
    /// the input data.
    ///
    /// Callers that process streaming input may be interested in retrying the
    /// deserialization once more data is available.
    pub fn is_eof(&self) -> bool {
        self.classify() == Category::Eof
    }
}

impl Error {
    #[cold]
    pub(crate) fn syntax(code: ErrorCode, line: usize, column: usize) -> Self {
        Self {
            err: Box::new(ErrorImpl { code, line, column }),
        }
    }

    #[cold]
    pub(crate) fn io(error: io::Error) -> Self {
        Self {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Io(error),
                line: 0,
                column: 0,
            }),
        }
    }
}

// Remove two layers of verbosity from the debug representation. Humans often
// end up seeing this representation because it is what unwrap() shows.
impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error({:?}, line: {}, column: {})",
            self.err.code.to_string(),
            self.err.line,
            self.err.column
        )
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&*self.err, f)
    }
}

impl serde::de::StdError for Error {
    #[cfg(feature = "std")]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.err.code {
            ErrorCode::Io(err) => err.source(),
            _ => None,
        }
    }
}

impl de::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        make_error(msg.to_string())
    }

    #[cold]
    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        Error::custom(format_args!(
            "invalid type: {}, expected {}",
            VfdUnexpected(unexp),
            exp,
        ))
    }

    #[cold]
    fn invalid_value(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        Error::custom(format_args!(
            "invalid value: {}, expected {}",
            VfdUnexpected(unexp),
            exp,
        ))
    }
}

impl ser::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        make_error(msg.to_string())
    }
}

#[cfg(feature = "std")]
impl From<Error> for io::Error {
    fn from(value: Error) -> Self {
        if let ErrorCode::Io(err) = value.err.code {
            err
        } else {
            match value.classify() {
                Category::Io => unreachable!(),
                Category::Syntax | Category::Data => io::Error::new(ErrorKind::InvalidData, value),
                Category::Eof => io::Error::new(ErrorKind::UnexpectedEof, value),
            }
        }
    }
}

/// Categorizes the cause of a `serde_vdf::Error`.
#[derive(PartialEq)]
pub enum Category {
    /// The error was caused by a failure to read or write bytes on an I/O stream
    Io,

    /// The error was caused by input that was not syntactically valid JSON.
    Syntax,

    /// The error was caused by input data that was semantically incorrect.
    Data,

    /// The error was caused by prematurely reaching the end of the input data.
    ///
    /// Callers that process streaming input may be interested in retrying the
    /// deserialization once more data is available.
    Eof,
}

struct ErrorImpl {
    code: ErrorCode,
    line: usize,
    column: usize,
}

impl Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.line == 0 {
            Display::fmt(&self.code, f)
        } else {
            write!(
                f,
                "{} at line {} column {}",
                self.code, self.line, self.column
            )
        }
    }
}

pub(crate) enum ErrorCode {
    /// Catchall for syntax error messages
    Message(Box<str>),

    /// Some I/O error occurred while serializing or deserializing.
    Io(io::Error),

    /// EOF while parsing an object.
    EofWhileParsingObject,

    /// EOF while parsing a string.
    EofWhileParsingString,

    /// Expected this character to start a JSON value.
    ExpectedSomeValue,

    /// Invalid unicode code point.
    InvalidUnicodeCodePoint,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::Message(msg) => f.write_str(msg),
            ErrorCode::Io(err) => Display::fmt(err, f),
            ErrorCode::EofWhileParsingObject => f.write_str("EOF while parsing an object"),
            ErrorCode::EofWhileParsingString => f.write_str("EOF while parsing a string"),
            ErrorCode::ExpectedSomeValue => f.write_str("expected value"),
            ErrorCode::InvalidUnicodeCodePoint => f.write_str("invalid unicode code point"),
        }
    }
}

struct VfdUnexpected<'a>(de::Unexpected<'a>);

impl<'a> Display for VfdUnexpected<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            de::Unexpected::Unit => formatter.write_str("null"),
            de::Unexpected::Float(value) => write!(
                formatter,
                "floating point `{}`",
                ryu::Buffer::new().format(value),
            ),
            unexp => Display::fmt(&unexp, formatter),
        }
    }
}

// Parse our own error message that looks like "{} at line {} column {}" to work
// around erased-serde round-tripping the error through de::Error::custom.
fn make_error(mut msg: String) -> Error {
    let (line, column) = parse_line_col(&mut msg).unwrap_or((0, 0));
    Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Message(msg.into_boxed_str()),
            line,
            column,
        }),
    }
}

fn parse_line_col(msg: &mut String) -> Option<(usize, usize)> {
    let start_of_suffix = match msg.rfind(" at line ") {
        Some(index) => index,
        None => return None,
    };

    // Find start and end of line number.
    let start_of_line = start_of_suffix + " at line ".len();
    let mut end_of_line = start_of_line;
    while starts_with_digit(&msg[end_of_line..]) {
        end_of_line += 1;
    }

    if !msg[end_of_line..].starts_with(" column ") {
        return None;
    }

    // Find start and end of column number.
    let start_of_column = end_of_line + " column ".len();
    let mut end_of_column = start_of_column;
    while starts_with_digit(&msg[end_of_column..]) {
        end_of_column += 1;
    }

    if end_of_column < msg.len() {
        return None;
    }

    // Parse numbers.
    let line = match usize::from_str(&msg[start_of_line..end_of_line]) {
        Ok(line) => line,
        Err(_) => return None,
    };
    let column = match usize::from_str(&msg[start_of_column..end_of_column]) {
        Ok(column) => column,
        Err(_) => return None,
    };

    msg.truncate(start_of_suffix);
    Some((line, column))
}

fn starts_with_digit(slice: &str) -> bool {
    match slice.as_bytes().first() {
        None => false,
        Some(&byte) => byte >= b'0' && byte <= b'9',
    }
}
