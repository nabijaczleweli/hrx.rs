//! Individual parsing primitives.
//!
//! The [`HrxArchive::from_str()`](../struct.HrxArchive.html#impl-FromStr) funxion is preferred to using these directly.
//!
//! In funxions which take the `boundary_length` argument,
//! that value specifies the amount of `=` characters in the archive boundary.
//!
//! However, due to the parser generator used, the `path()` funxion also takes it, despite it being unused.
//! It can be safely ignored in that case.
//!
//! # Grammar
//!
//! The monoverbial funxions in this module represent the nodes in the grammar,
//! copied verbatim from the [google/hrx](https://github.com/google/hrx) repository:
//!
//! ```plaintext
//! archive        ::= entry* comment?
//!
//! entry          ::= comment? (file | directory)
//! comment        ::= boundary newline body
//! file           ::= boundary " "+ path newline body?
//! directory      ::= boundary " "+ path "/" newline+
//! boundary       ::= "<" "="+ ">" // must exactly match the first boundary in the archive
//! newline        ::= U+000A LINE FEED
//! body           ::= contents newline // no newline at the end of the archive (if the
//!                                     // archive ends in a body, all trailing
//!                                     // newlines are part of that body's contents)
//! contents       ::= any sequence of characters that neither begins with boundary nor
//!                    includes U+000A LINE FEED followed immediately by boundary
//!
//! path           ::= path-component ("/" path-component)*
//! path-component ::= path-character+ // not equal to "." or ".."
//! path-character ::= any character other than U+0000 through U+001F, U+007F DELETE, U+002F
//!                    SOLIDUS, U+003A COLON, or U+005C REVERSE SOLIDUS
//! ```


mod individual;
mod grammar;

pub use self::grammar::{ParseResult, ParseError, directory, archive, comment, entry, body, file, path};
pub use self::individual::{reduce_raw_entries_and_validate_directory_tree, discover_first_boundary_length};
