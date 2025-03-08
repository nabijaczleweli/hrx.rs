use std::num::NonZeroUsize;

pub use self::hrx::*;

peg::parser!{grammar hrx(boundary_length: NonZeroUsize) for str {

use self::super::super::super::{HrxEntryData, HrxEntry, HrxPath};


/// `entry* comment?`
///
/// Note, that this does not perform any content validation beyond grammar.
pub rule archive() -> (Option<String>, Vec<(HrxPath, HrxEntry)>, NonZeroUsize)
    = ee:entry()* c:comment()?
    { (c.map(str::to_string), ee, boundary_length) }

/// `comment? (file | directory)`
pub rule entry() -> (HrxPath, HrxEntry)
    = c:comment()? d:file()
    {
        (
            d.0,
            HrxEntry {
                comment: c.map(str::to_string),
                data: HrxEntryData::File {
                    body: d.1.map(str::to_string),
                },
            }
        )
    }
    / c:comment()? d:directory()
    {
        (
            d,
            HrxEntry {
                comment: c.map(str::to_string),
                data: HrxEntryData::Directory,
            }
        )
    }

/// `boundary newline body`
pub rule comment() -> &'input str
    = boundary() newline() b:body()
    { b }

/// `boundary " "+ path newline body?`
pub rule file() -> (HrxPath, Option<&'input str>)
    = boundary() " "+ p:path() newline() b:body()?
    { (p, b) }

/// `boundary " "+ path "/" newline+`
pub rule directory() -> HrxPath
    = boundary() " "+ p:path() "/" newline()+
    { p }

/// `"<" "="+ ">"`
///
/// Must exactly match the first boundary in the archive
rule boundary()
    = "<" "="*<{boundary_length.get()}> ">"

/// U+000A LINE FEED
rule newline()
    = "\n"

/// `contents newline`
///
/// No newline at the end of the archive (if the archive ends in a body, all trailing newlines are part of that body's contents)
///
/// NB: this doesn't explicitly consume a newline as that has been moved down to `contents`.
pub rule body() -> &'input str
    = contents()

/// Any sequence of characters that neither begins with `boundary` nor includes U+000A LINE FEED followed immediately by `boundary`
///
/// The first pattern stolen from the `@E` "Search loop" expansion from here https://nim-lang.org/docs/pegs.html
rule contents() -> &'input str
    = !boundary() ctnt:$((!(newline() boundary()) [_])+) newline() &boundary() { ctnt }
    / !boundary() ctnt:$([_]+)                                                 { ctnt }


/// `path-component ("/" path-component)*`
pub rule path() -> HrxPath
    = p:$(path_component() ("/" path_component())*)
    { HrxPath(p.to_string()) }

/// `path-character+`
///
/// Not equal to `"."` or `".."`
rule path_component() -> &'input str
    = pc:$(path_character()+)
    {?
        if pc == "." || pc == ".." {
            Err("Invalid '.' or '..' path component")
        } else {
            Ok(pc)
        }
    }

/// Any character other than U+0000 through U+001F, U+007F DELETE, U+002F SOLIDUS, U+003A COLON, or U+005C REVERSE SOLIDUS
rule path_character() -> char
    = quiet!{c:$(!['\x00'..='\x1F' | '\x7F' | '/' | ':' | '\\'][_]) { c.chars().next().unwrap() }}
    / expected!("Any character other than U+0000 through U+001F, U+007F DELETE, U+002F SOLIDUS, U+003A COLON, or U+005C REVERSE SOLIDUS")

}}
