mod invalid;

use hrx::{HrxEntryData, HrxArchive, HrxEntry};
use std::num::NonZeroUsize;
use std::str::FromStr;


/// This is the example/comment-only.hrx file in the original google/hrx repository.
#[test]
fn comment_only() {
    let arch_str = r#"<===>
A HRX file may consist of only a comment and nothing else.
"#;

    let mut arch = HrxArchive::new(NonZeroUsize::new(3).unwrap());
    arch.comment = Some("A HRX file may consist of only a comment and nothing else.\n".to_string());

    assert_eq!(HrxArchive::from_str(arch_str), Ok(arch));
}

/// This is the example/comments.hrx file in the original google/hrx repository.
#[test]
fn comments() {
    let arch_str = r#"<===>
This is a comment.
<===> file1
This is the contents of the file.

<===>
This is another comment.
<===> file2
This is the contents of another file.

"#;

    let mut arch = HrxArchive::new(NonZeroUsize::new(3).unwrap());
    arch.entries.insert("file1".parse().unwrap(),
                        HrxEntry {
                            comment: Some("This is a comment.".to_string()),
                            data: HrxEntryData::File { body: Some("This is the contents of the file.\n".to_string()) },
                        });
    arch.entries.insert("file2".parse().unwrap(),
                        HrxEntry {
                            comment: Some("This is another comment.".to_string()),
                            data: HrxEntryData::File { body: Some("This is the contents of another file.\n\n".to_string()) },
                        });

    assert_eq!(HrxArchive::from_str(arch_str), Ok(arch));
}

/// This is the example/complex-filenames.hrx file in the original google/hrx repository.
#[test]
fn complex_filenames() {
    let arch_str = r#"<===> .dir/.../.file
Filenames may contain dots, as long as they're not "." or "..".

<===> ~`!@#$%^&*()_-+= {}[]|;"'<,>.?
Filenames can contain all kinds of weird characters.

<===> ☃
Non-ASCII Unicode names are allowed.
"#;

    let mut arch = HrxArchive::new(NonZeroUsize::new(3).unwrap());
    arch.entries.insert(".dir/.../.file".parse().unwrap(),
                        HrxEntry {
                            comment: None,
                            data: HrxEntryData::File { body: Some("Filenames may contain dots, as long as they're not \".\" or \"..\".\n".to_string()) },
                        });
    arch.entries.insert("~`!@#$%^&*()_-+= {}[]|;\"'<,>.?".parse().unwrap(),
                        HrxEntry {
                            comment: None,
                            data: HrxEntryData::File { body: Some("Filenames can contain all kinds of weird characters.\n".to_string()) },
                        });
    arch.entries.insert("☃".parse().unwrap(),
                        HrxEntry {
                            comment: None,
                            data: HrxEntryData::File { body: Some("Non-ASCII Unicode names are allowed.\n".to_string()) },
                        });

    assert_eq!(HrxArchive::from_str(arch_str), Ok(arch));
}

/// This is the example/directory.hrx file in the original google/hrx repository.
#[test]
fn directory() {
    let arch_str = r#"<===>
We know this is a directory because it ends with "/".
<===> dir/
<===>
Directories can be nested, too.
<===> dir/subdir/

<===>
Parent directories don't have to be defined explicitly.
<===> other/subdir/
"#;

    let mut arch = HrxArchive::new(NonZeroUsize::new(3).unwrap());
    arch.entries.insert("dir".parse().unwrap(),
                        HrxEntry {
                            comment: Some("We know this is a directory because it ends with \"/\".".to_string()),
                            data: HrxEntryData::Directory,
                        });
    arch.entries.insert("dir/subdir".parse().unwrap(),
                        HrxEntry {
                            comment: Some("Directories can be nested, too.".to_string()),
                            data: HrxEntryData::Directory,
                        });
    arch.entries.insert("other/subdir".parse().unwrap(),
                        HrxEntry {
                            comment: Some("Parent directories don't have to be defined explicitly.".to_string()),
                            data: HrxEntryData::Directory,
                        });

    assert_eq!(HrxArchive::from_str(arch_str), Ok(arch));
}

/// This is the example/empty-file.hrx file in the original google/hrx repository.
#[test]
fn empty_file() {
    let arch_str = r#"<===>
This file is empty.
<===> file1
<===>
So is this one.
<===> file2
"#;

    let mut arch = HrxArchive::new(NonZeroUsize::new(3).unwrap());
    arch.entries.insert("file1".parse().unwrap(),
                        HrxEntry {
                            comment: Some("This file is empty.".to_string()),
                            data: HrxEntryData::File { body: None },
                        });
    arch.entries.insert("file2".parse().unwrap(),
                        HrxEntry {
                            comment: Some("So is this one.".to_string()),
                            data: HrxEntryData::File { body: None },
                        });

    assert_eq!(HrxArchive::from_str(arch_str), Ok(arch));
}

/// This is the example/files-in-directories.hrx file in the original google/hrx repository.
#[test]
fn files_in_directories() {
    let arch_str = r#"<===> dir/file1
This file is in a directory. Directories implicitly exist once there are any
files in them.
<===> path/to/file2
This file is in a deeper directory.
"#;

    let mut arch = HrxArchive::new(NonZeroUsize::new(3).unwrap());
    arch.entries.insert("dir/file1".parse().unwrap(),
                        HrxEntry {
                            comment: None,
                            data: HrxEntryData::File {
                                body: Some("This file is in a directory. Directories implicitly exist once there are any\nfiles in them.".to_string()),
                            },
                        });
    arch.entries.insert("path/to/file2".parse().unwrap(),
                        HrxEntry {
                            comment: None,
                            data: HrxEntryData::File { body: Some("This file is in a deeper directory.\n".to_string()) },
                        });

    assert_eq!(HrxArchive::from_str(arch_str), Ok(arch));
}

/// This is the example/inline-boundary.hrx file in the original google/hrx repository.
#[test]
fn inline_boundary() {
    let arch_str = r#"<===> file
This <===> doesn't count as a boundary because it's not on its own line.
"#;

    let mut arch = HrxArchive::new(NonZeroUsize::new(3).unwrap());
    arch.entries.insert("file".parse().unwrap(),
                        HrxEntry {
                            comment: None,
                            data: HrxEntryData::File { body: Some("This <===> doesn't count as a boundary because it's not on its own line.\n".to_string()) },
                        });

    assert_eq!(HrxArchive::from_str(arch_str), Ok(arch));
}

/// This is the example/nested.hrx file in the original google/hrx repository.
#[test]
fn nested() {
    let arch_str = r#"<===> file1.hrx
<=====> nested-file1.hrx
This is a HRX file nested within a HRX file.

<=====> nested-file2.hrx
You can tell it's not part of the outer file because the boundaries are longer.

<===> file2.hrx
<=> nested-file1.hrx
Inner files can also contain shorter boundaries...

<=> nested-file2.hrx
...as long as they don't contain the outer file's boundary.
"#;

    let mut arch = HrxArchive::new(NonZeroUsize::new(3).unwrap());
    arch.entries.insert("file1.hrx".parse().unwrap(),
                        HrxEntry {
                            comment: None,
                            data: HrxEntryData::File {
                                body: Some("<=====> nested-file1.hrx
This is a HRX file nested within a HRX file.

<=====> nested-file2.hrx
You can tell it's not part of the outer file because the boundaries are longer.
"
                                    .to_string()),
                            },
                        });
    arch.entries.insert("file2.hrx".parse().unwrap(),
                        HrxEntry {
                            comment: None,
                            data: HrxEntryData::File {
                                body: Some("<=> nested-file1.hrx
Inner files can also contain shorter boundaries...

<=> nested-file2.hrx
...as long as they don't contain the outer file's boundary.
"
                                    .to_string()),
                            },
                        });

    assert_eq!(HrxArchive::from_str(arch_str), Ok(arch));
}

/// This is the example/no-trailing-newlines.hrx file in the original google/hrx repository.
#[test]
fn no_trailing_newlines() {
    let arch_str = r#"<===> file1
This file doesn't have a trailing newline.
<===> file2
Neither does this one."#;

    let mut arch = HrxArchive::new(NonZeroUsize::new(3).unwrap());
    arch.entries.insert("file1".parse().unwrap(),
                        HrxEntry {
                            comment: None,
                            data: HrxEntryData::File { body: Some("This file doesn't have a trailing newline.".to_string()) },
                        });
    arch.entries.insert("file2".parse().unwrap(),
                        HrxEntry {
                            comment: None,
                            data: HrxEntryData::File { body: Some("Neither does this one.".to_string()) },
                        });

    assert_eq!(HrxArchive::from_str(arch_str), Ok(arch));
}

/// This is the example/simple.hrx file in the original google/hrx repository.
#[test]
fn simple() {
    let arch_str = r#"<===> input.scss
ul {
  margin-left: 1em;
  li {
    list-style-type: none;
  }
}

<===> output.css
ul {
  margin-left: 1em;
}
ul li {
  list-style-type: none;
}
"#;

    let mut arch = HrxArchive::new(NonZeroUsize::new(3).unwrap());
    arch.entries.insert("input.scss".parse().unwrap(),
                        HrxEntry {
                            comment: None,
                            data: HrxEntryData::File { body: Some("ul {\n  margin-left: 1em;\n  li {\n    list-style-type: none;\n  }\n}\n".to_string()) },
                        });
    arch.entries.insert("output.css".parse().unwrap(),
                        HrxEntry {
                            comment: None,
                            data: HrxEntryData::File { body: Some("ul {\n  margin-left: 1em;\n}\nul li {\n  list-style-type: none;\n}\n".to_string()) },
                        });

    assert_eq!(HrxArchive::from_str(arch_str), Ok(arch));
}

/// This is the example/trailing-comment.hrx file in the original google/hrx repository.
#[test]
fn trailing_comment() {
    let arch_str = r#"<===> file.hrx
The contents of a file.

<===>
A comment may appear at the end of a file.
"#;

    let mut arch = HrxArchive::new(NonZeroUsize::new(3).unwrap());
    arch.entries.insert("file.hrx".parse().unwrap(),
                        HrxEntry {
                            comment: None,
                            data: HrxEntryData::File { body: Some("The contents of a file.\n".to_string()) },
                        });
    arch.comment = Some("A comment may appear at the end of a file.\n".to_string());

    assert_eq!(HrxArchive::from_str(arch_str), Ok(arch));
}
