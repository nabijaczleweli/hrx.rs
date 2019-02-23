//! This is an adaptation of the example/invalid/invalid-boundaries.hrx file in the original google/hrx repository.


use hrx::{HrxArchive, HrxError};
use std::str::FromStr;


#[test]
fn none() {
    assert_eq!(HrxArchive::from_str("A HRX file must begin with a boundary.\n"), Err(HrxError::NoBoundary));
}

#[test]
fn empty() {
    assert_eq!(HrxArchive::from_str("<>\n"), Err(HrxError::NoBoundary));
}

#[test]
fn unopened() {
    assert_eq!(HrxArchive::from_str("======>\n"), Err(HrxError::NoBoundary));
}

#[test]
fn unclosed() {
    assert_eq!(HrxArchive::from_str("<======\n"), Err(HrxError::NoBoundary));
}
