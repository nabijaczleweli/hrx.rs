//! This is an adaptation of the example/invalid/invalid-paths.hrx file in the original google/hrx repository.


use hrx::parse::{ParseError, path};
use std::num::NonZeroUsize;


#[test]
fn initial_slash() {
    assert_eq!(path("/file", NonZeroUsize::new(1).unwrap()),
               Err(ParseError {
                   line: 1,
                   column: 1,
                   offset: 0,
                   expected: vec!["Any character other than U+0000 through U+001F, U+007F DELETE, U+002F SOLIDUS, U+003A COLON, or U+005C REVERSE SOLIDUS"]
                       .into_iter()
                       .collect(),
               }));
}

#[test]
fn double_slash() {
    assert_eq!(path("dir//file", NonZeroUsize::new(1).unwrap()),
               Err(ParseError {
                   line: 1,
                   column: 5,
                   offset: 4,
                   expected: vec!["Any character other than U+0000 through U+001F, U+007F DELETE, U+002F SOLIDUS, U+003A COLON, or U+005C REVERSE SOLIDUS"]
                       .into_iter()
                       .collect(),
               }));
}

#[test]
fn final_slash() {
    assert_eq!(path("dir//", NonZeroUsize::new(1).unwrap()),
               Err(ParseError {
                   line: 1,
                   column: 5,
                   offset: 4,
                   expected: vec!["Any character other than U+0000 through U+001F, U+007F DELETE, U+002F SOLIDUS, U+003A COLON, or U+005C REVERSE SOLIDUS"]
                       .into_iter()
                       .collect(),
               }));
}

#[test]
fn single_dot() {
    assert_eq!(path(".", NonZeroUsize::new(1).unwrap()),
               Err(ParseError {
                   line: 1,
                   column: 2,
                   offset: 1,
                   expected: vec!["Invalid '.' or '..' path component",
                                  "Any character other than U+0000 through U+001F, U+007F DELETE, U+002F SOLIDUS, U+003A COLON, or U+005C REVERSE SOLIDUS"]
                       .into_iter()
                       .collect(),
               }));
}

#[test]
fn double_dot() {
    assert_eq!(path("..", NonZeroUsize::new(1).unwrap()),
               Err(ParseError {
                   line: 1,
                   column: 3,
                   offset: 2,
                   expected: vec!["Invalid '.' or '..' path component",
                                  "Any character other than U+0000 through U+001F, U+007F DELETE, U+002F SOLIDUS, U+003A COLON, or U+005C REVERSE SOLIDUS"]
                       .into_iter()
                       .collect(),
               }));
}

#[test]
fn single_dot_component() {
    assert_eq!(path("dir/./file", NonZeroUsize::new(1).unwrap()),
               Err(ParseError {
                   line: 1,
                   column: 6,
                   offset: 5,
                   expected: vec!["Invalid '.' or '..' path component",
                                  "Any character other than U+0000 through U+001F, U+007F DELETE, U+002F SOLIDUS, U+003A COLON, or U+005C REVERSE SOLIDUS"]
                       .into_iter()
                       .collect(),
               }));
}

#[test]
fn double_dot_component() {
    assert_eq!(path("dir/../file", NonZeroUsize::new(1).unwrap()),
               Err(ParseError {
                   line: 1,
                   column: 7,
                   offset: 6,
                   expected: vec!["Invalid '.' or '..' path component",
                                  "Any character other than U+0000 through U+001F, U+007F DELETE, U+002F SOLIDUS, U+003A COLON, or U+005C REVERSE SOLIDUS"]
                       .into_iter()
                       .collect(),
               }));
}

#[test]
fn backslash() {
    assert_eq!(path("dir\\file", NonZeroUsize::new(1).unwrap()),
               Err(ParseError {
                   line: 1,
                   column: 4,
                   offset: 3,
                   expected: vec!["/",
                                  "Any character other than U+0000 through U+001F, U+007F DELETE, U+002F SOLIDUS, U+003A COLON, or U+005C REVERSE SOLIDUS"]
                       .into_iter()
                       .collect(),
               }));
}

#[test]
fn invalid_ascii() {
    assert_eq!(path("\x7F", NonZeroUsize::new(1).unwrap()),
               Err(ParseError {
                   line: 1,
                   column: 1,
                   offset: 0,
                   expected: vec!["Any character other than U+0000 through U+001F, U+007F DELETE, U+002F SOLIDUS, U+003A COLON, or U+005C REVERSE SOLIDUS"]
                       .into_iter()
                       .collect(),
               }));
}

#[test]
fn colon() {
    assert_eq!(path("C:/file", NonZeroUsize::new(1).unwrap()),
               Err(ParseError {
                   line: 1,
                   column: 2,
                   offset: 1,
                   expected: vec!["/",
                                  "Any character other than U+0000 through U+001F, U+007F DELETE, U+002F SOLIDUS, U+003A COLON, or U+005C REVERSE SOLIDUS"]
                       .into_iter()
                       .collect(),
               }));
}

// #[test]
// fn no_space_before_path() {
//     <======>file
// }
