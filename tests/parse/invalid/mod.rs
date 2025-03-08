mod invalid_boundaries;
mod duplicates;
mod grammar;

use hrx::{HrxArchive, HrxError};
use std::collections::BTreeSet;
use hrx::parse::ParseError;
use std::str::FromStr;


#[derive(Debug, PartialEq, Eq)]
pub struct VisibleParseError {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub expected: BTreeSet<&'static str>,
}
impl From<ParseError> for VisibleParseError {
    fn from(pe: ParseError) -> Self {
        Self {
            line: pe.line,
            column: pe.column,
            offset: pe.offset,
            expected: pe.expected.tokens().collect(),
        }
    }
}


fn force_parse_error(he: HrxError) -> VisibleParseError {
    match he {
        HrxError::Parse(pe) => pe.into(),
        _ => panic!(),
    }
}

/// This is the example/invalid/multi-comment.hrx file in the original google/hrx repository.
#[test]
fn multi_comment() {
    let arch_str = r#"<===>
A comment can't be followed by another comment.
<===>
"#;

    assert_eq!(HrxArchive::from_str(arch_str).map_err(force_parse_error),
               Err(VisibleParseError {
                   line: 3,
                   column: 6,
                   offset: 59,
                   expected: vec!["\" \""].into_iter().collect(),
               }));
}

/// This is the example/invalid/directory-contents.hrx file in the original google/hrx repository.
#[test]
fn directory_contents() {
    let arch_str = r#"<===> dir/
A directory can't have text contents.
"#;

    assert_eq!(HrxArchive::from_str(arch_str).map_err(force_parse_error),
               Err(VisibleParseError {
                   line: 2,
                   column: 1,
                   offset: 11,
                   expected: vec!["\"<\"", "\"\\n\"", "EOF"].into_iter().collect(),
               }));
}
