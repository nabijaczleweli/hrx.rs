use self::super::parse;
use std::error::Error;
use lazysort::Sorted;
use std::fmt;


/// Generic error type, encompassing more precise errors.
///
/// # Examples
///
/// ```
/// # use hrx::{HrxArchive, HrxError};
/// # use hrx::parse::ParseError;
/// # use std::str::FromStr;
/// assert_eq!(HrxArchive::from_str("Not an actual archive, missing a boundary"),
///            Err(HrxError::NoBoundary));
///
/// let err = HrxArchive::from_str("<====>no space before").unwrap_err();
/// assert_eq!(err, HrxError::Parse(ParseError {
///     line: 1,
///     column: 7,
///     offset: 6,
///     expected: vec![" ", "\n"].into_iter().collect(),
/// }));
/// assert_eq!(err.to_string(), r#"Parse failed at 1:7 [position 6]: expected "\n", or " "."#);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HrxError {
    /// No valid HRX boundary found
    NoBoundary,
    /// An error occured during parsing
    Parse(parse::ParseError),
    /// A body was made to contain the archive boundary. Deserialising the archive would *not* work
    BodyContainsBoundary(ErroneousBodyPath),
}

/// A path to a `body` containing an invalid sequence
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErroneousBodyPath {
    /// The root archive comment
    RootComment,
    /// A comment to the entry with the specified path
    EntryComment(String),
    /// The data of the entry with the specified path
    EntryData(String),
}


impl From<parse::ParseError> for HrxError {
    fn from(pe: parse::ParseError) -> HrxError {
        HrxError::Parse(pe)
    }
}

impl From<ErroneousBodyPath> for HrxError {
    fn from(bcb: ErroneousBodyPath) -> HrxError {
        HrxError::BodyContainsBoundary(bcb)
    }
}

impl fmt::Display for HrxError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &HrxError::NoBoundary => fmt.write_str("No boundary found")?,
            &HrxError::Parse(ref pe) => {
                write!(fmt, "Parse failed at {}:{} [position {}]: expected ", pe.line, pe.column, pe.offset)?;

                for (i, x) in pe.expected.iter().sorted().enumerate() {
                    if i != 0 {
                        fmt.write_str(", ")?;

                        if i == pe.expected.len() - 1 {
                            fmt.write_str("or ")?;
                        }
                    }

                    if x.len() == 1 {
                        write!(fmt, "{:?}", x)?;
                    } else {
                        fmt.write_str(x)?;
                    }
                }

                fmt.write_str(".")?;
            }
            &HrxError::BodyContainsBoundary(ref path) => {
                match path {
                    ErroneousBodyPath::RootComment => fmt.write_str("Root archive comment")?,
                    ErroneousBodyPath::EntryComment(ref pp) => write!(fmt, "Comment for \"{}\" entry", pp)?,
                    ErroneousBodyPath::EntryData(ref pp) => write!(fmt, "Data of \"{}\" entry", pp)?,
                }
                fmt.write_str(" contains the archive boundary, making resulting archive not redeserialisable.")?;
            }
        }

        Ok(())
    }
}

impl Error for HrxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            &HrxError::Parse(ref pe) => Some(pe),
            _ => None,
        }
    }
}
