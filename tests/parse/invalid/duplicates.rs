//! This is an adaptation of the example/invalid/duplicates.hrx file in the original google/hrx repository.


use hrx::{HrxArchive, HrxError};
use std::str::FromStr;


#[test]
fn duplicate_files() {
    assert_eq!(HrxArchive::from_str("<======> file\n<======> file\n"),
               Err(HrxError::DuplicateEntry("file".to_string())));
}

#[test]
fn duplicate_dirs() {
    assert_eq!(HrxArchive::from_str("<======> dir/\n<======> dir/\n"),
               Err(HrxError::DuplicateEntry("dir".to_string())));
}

#[test]
fn file_as_parent() {
    assert_eq!(HrxArchive::from_str("<======> file\n<======> file/sub\n"),
               Err(HrxError::FileAsDirectory("file".to_string(), "file/sub".to_string())));
}
