mod invalid_boundaries;
mod duplicates;
mod main;

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
