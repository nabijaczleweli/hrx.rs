//! A Rust implementation of the HRX plain text archive format.
//!
//! Consult the [`google/hrx`](//github.com/google/hrx) repo for the details on the format.
//!
//! # Examples
//!
//! ```
//! # use hrx::{HrxEntryData, HrxArchive, HrxEntry, HrxError, HrxPath};
//! # use std::str::FromStr;
//! let input_text = "<===> input.scss
//! ul {
//!   li {
//!     list-style: none;
//!   }
//! }
//!
//! <===>
//! Generated files
//! <===> out/
//!
//! <===> out/input.css
//! ul li {
//!   list-style: none;
//! }
//! ";
//!
//! let mut archive = HrxArchive::from_str(input_text)?;
//!
//! assert_eq!(archive.comment, None);
//! assert_eq!(archive.entries,
//!            [(HrxPath::from_str("input.scss")?,
//!              HrxEntry {
//!                 comment: None,
//!                 data: HrxEntryData::File {
//!                     body: Some("ul {\n  li {\n    list-style: none;\n  }\n}\n".to_string()),
//!                 },
//!              }),
//!             (HrxPath::from_str("out")?,
//!              HrxEntry {
//!                 comment: Some("Generated files".to_string()),
//!                 data: HrxEntryData::Directory,
//!              }),
//!             (HrxPath::from_str("out/input.css")?,
//!              HrxEntry {
//!                 comment: None,
//!                 data: HrxEntryData::File {
//!                     body: Some("ul li {\n  list-style: none;\n}\n".to_string()),
//!                 },
//!              })].iter().cloned().collect());
//!
//! archive.entries.remove(&HrxPath::from_str("out")?);
//! archive.comment = Some("Snapshot of commit 264a050c".to_string());
//!
//! let mut out = vec![];
//! archive.serialise(&mut out).unwrap();
//!
//! assert_eq!(String::from_utf8(out).unwrap(), "<===> input.scss
//! ul {
//!   li {
//!     list-style: none;
//!   }
//! }
//!
//! <===> out/input.css
//! ul li {
//!   list-style: none;
//! }
//!
//! <===>
//! Snapshot of commit 264a050c");
//! # Ok::<(), HrxError>(())
//! ```
//!
//! # Special thanks
//!
//! To all who support further development on [Patreon](https://patreon.com/nabijaczleweli), in particular:
//!
//!   * ThePhD
//!   * Embark Studios
//!   * Jasper Bekkers


extern crate linked_hash_map;
extern crate lazysort;
extern crate memchr;

pub mod util;
pub mod parse;

mod repr;
mod error;
mod output;

pub use self::error::{ErroneousBodyPath, HrxError};
pub use self::repr::{HrxEntryData, HrxArchive, HrxEntry, HrxPath};
