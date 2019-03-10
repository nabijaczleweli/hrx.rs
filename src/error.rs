use std::fmt::{self, Write};
use self::super::parse;
use std::error::Error;
use lazysort::Sorted;


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
    /// Some `body`s were made to contain the archive boundary. Deserialising the archive wouldn't work as expected
    BodyContainsBoundary(Vec<ErroneousBodyPath>),
    /// Two entries share the same path
    DuplicateEntry(String),
    /// An entry attempted to use a file as a directory
    FileAsDirectory(String, String),
}

/// A path to a `body` which contains an invalid sequence
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
       HrxError::BodyContainsBoundary(vec![bcb])
   }
}

impl<V: Into<Vec<ErroneousBodyPath>>> From<V> for HrxError {
    fn from(paths: V) -> HrxError {
        HrxError::BodyContainsBoundary(paths.into())
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
            &HrxError::BodyContainsBoundary(ref paths) => {
                fn first_char(fmt: &mut fmt::Formatter, c: char, first: bool, last: bool) -> fmt::Result {
                    if first {
                        write!(fmt, "{}", c.to_uppercase())?
                    } else {
                        fmt.write_str(", ")?;
                        if last {
                            fmt.write_str("and ")?;
                        }
                        fmt.write_char(c)?;
                    }

                    Ok(())
                }


                if !paths.is_empty() {
                    for (i, path) in paths.iter().enumerate() {
                        let first = i == 0;
                        let last = i == paths.len();
                        match path {
                            ErroneousBodyPath::RootComment => {
                                first_char(fmt, 'r', first, last)?;
                                fmt.write_str("oot archive comment")?
                            }
                            ErroneousBodyPath::EntryComment(ref pp) => {
                                first_char(fmt, 'c', first, last)?;
                                write!(fmt, "omment for \"{}\" entry", pp)?
                            }
                            ErroneousBodyPath::EntryData(ref pp) => {
                                first_char(fmt, 'd', first, last)?;
                                write!(fmt, "ata of \"{}\" entry", pp)?
                            }
                        }
                    }
                    fmt.write_str(" contain")?;
                    if paths.len() != 1 {
                        fmt.write_char('s')?;
                    }
                    fmt.write_str(" the archive boundary, making resulting archive not redeserialisable.")?;
                } else {
                    fmt.write_str("No paths specified.")?;
                }
            }
            &HrxError::DuplicateEntry(ref path) => {
                fmt.write_str("Duplicate entry: ")?;
                fmt.write_str(&path)?;
            }
            &HrxError::FileAsDirectory(ref file, ref who) => {
                fmt.write_str(&who)?;
                fmt.write_str(" attempted to use the file at ")?;
                fmt.write_str(&file)?;
                fmt.write_str(" as a directrory.")?;
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
