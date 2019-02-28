use self::super::{HrxEntryData, HrxArchive, HrxError};
use std::io::{Error as IoError, Write};
use self::super::util::boundary_str;


/// `IoError` is absolute garbage when it comes to usability when it's a variant
enum CompoundError {
    Hrx(HrxError),
    Io(IoError),
}

impl From<HrxError> for CompoundError {
    fn from(hrx: HrxError) -> CompoundError {
        CompoundError::Hrx(hrx)
    }
}

impl From<IoError> for CompoundError {
    fn from(io: IoError) -> CompoundError {
        CompoundError::Io(io)
    }
}

impl From<CompoundError> for Result<HrxError, IoError> {
    fn from(ce: CompoundError) -> Result<HrxError, IoError> {
        match ce {
            CompoundError::Hrx(hrx) => Ok(hrx),
            CompoundError::Io(io) => Err(io),
        }
    }
}


pub fn write_archive<W: Write>(ar: &HrxArchive, into: &mut W) -> Result<(), Result<HrxError, IoError>> {
    write_archive_impl(ar, into)?;
    Ok(())
}

fn write_archive_impl<W: Write>(ar: &HrxArchive, into: &mut W) -> Result<(), CompoundError> {
    ar.validate_content()?;

    let bound = boundary_str(ar.boundary_length);
    let mut first_bound = true;
    let mut ending_newline = false;

    for (p, e) in &ar.entries {
        write_comment(&e.comment, &bound, &mut first_bound, into)?;

        write_bound(&bound, &mut first_bound, into)?;
        into.write_all(&[b' '])?;
        into.write_all(p.0.as_bytes())?;
        match e.data {
            HrxEntryData::File { body: None } => {
                ending_newline = true;
            }
            HrxEntryData::File { body: Some(ref body) } if body.is_empty() => {
                ending_newline = true;
            }
            HrxEntryData::File { body: Some(ref body) } => {
                into.write_all(&[b'\n'])?;
                into.write_all(body.as_bytes())?;

                ending_newline = false;
            }
            HrxEntryData::Directory => {
                into.write_all(&[b'/'])?;
                ending_newline = true;
            }
        }
    }

    if !write_comment(&ar.comment, &bound, &mut first_bound, into)? && ending_newline {
        into.write_all(&[b'\n'])?;
    }

    Ok(())
}

fn write_bound<W: Write>(bound: &str, first_bound: &mut bool, into: &mut W) -> Result<(), CompoundError> {
    if *first_bound {
        into.write_all(bound[1..].as_bytes())?;

        *first_bound = false;
    } else {
        into.write_all(bound.as_bytes())?;
    }

    Ok(())
}

fn write_comment<W: Write>(comment: &Option<String>, bound: &str, first_bound: &mut bool, into: &mut W) -> Result<bool, CompoundError> {
    if let Some(cmt) = comment.as_ref() {
        write_bound(bound, first_bound, into)?;
        into.write_all(&[b'\n'])?;
        into.write_all(cmt.as_bytes())?;

        Ok(true)
    } else {
        Ok(false)
    }
}
