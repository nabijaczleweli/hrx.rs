use self::super::grammar;
use std::error::Error;
use std::fmt;


/// Generic error type, encompassing more precise errors.
///
/// # Examples
///
/// ```
/// # use hrx::{HrxArchive, HrxError};
/// # use hrx::grammar::ParseError;
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
/// assert_eq!(err.to_string(), r#"Parse failed at 1:7 [position 6]: expected " ", or "\n"."#);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HrxError {
    /// No valid HRX boundary found
    NoBoundary,
    /// An error occured during parsing
    Parse(grammar::ParseError),
}

impl From<grammar::ParseError> for HrxError {
    fn from(pe: grammar::ParseError) -> HrxError {
        HrxError::Parse(pe)
    }
}

impl fmt::Display for HrxError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &HrxError::NoBoundary => fmt.write_str("No boundary found")?,
            &HrxError::Parse(ref pe) => {
                write!(fmt, "Parse failed at {}:{} [position {}]: expected ", pe.line, pe.column, pe.offset)?;

                for (i, x) in pe.expected.iter().enumerate() {
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
        }

        Ok(())
    }
}

impl Error for HrxError {
    fn source(&self)-> Option<&(dyn Error + 'static)> {
        match self {
            &HrxError::NoBoundary => None,
            &HrxError::Parse(ref pe) => Some(pe),
        }
    }
}
