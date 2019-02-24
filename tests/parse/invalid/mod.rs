mod invalid_boundaries;
mod duplicates;
mod grammar;

use hrx::parse::ParseError;
use std::str::FromStr;
use hrx::HrxArchive;


/// This is the example/invalid/multi-comment.hrx file in the original google/hrx repository.
#[test]
fn multi_comment() {
    let arch_str = r#"<===>
A comment can't be followed by another comment.
<===>
"#;

    assert_eq!(HrxArchive::from_str(arch_str),
               Err(ParseError {
                       line: 3,
                       column: 6,
                       offset: 59,
                       expected: vec![" "]
                           .into_iter()
                           .collect(),
                   }
                   .into()));
}

/// This is the example/invalid/directory-contents.hrx file in the original google/hrx repository.
#[test]
fn directory_contents() {
    let arch_str = r#"<===> dir/
A directory can't have text contents.
"#;

    assert_eq!(HrxArchive::from_str(arch_str),
               Err(ParseError {
                       line: 2,
                       column: 1,
                       offset: 11,
                       expected: vec!["<", "\n"]
                           .into_iter()
                           .collect(),
                   }
                   .into()));
}
