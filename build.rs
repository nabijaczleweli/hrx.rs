extern crate rustfmt;
extern crate peg;

use std::io::{BufReader, BufRead, Write};
use std::collections::BTreeMap;
use std::path::{PathBuf, Path};
use std::fs::{self, File};
use std::env;


fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR env var nonexistant/non-Unicode");
    let doc_correction_map = {
        let mut m = BTreeMap::new();
        m.insert("pub struct ParseError {", "HRX parsing error");
        m.insert("    pub line: usize,", "1-based line # of error");
        m.insert("    pub column: usize,", "1-based column # of error");
        m.insert("    pub offset: usize,", "Byte offset of error");
        m.insert("    pub expected: ::std::collections::HashSet<&'static str>,", "Expected but unmatched rules");
        m.insert("pub type ParseResult<T> = Result<T, ParseError>;", "Convenience result type");
        m
    };


    let mut doc_buf = vec![];
    let mut doc_map = vec![];
    for line in BufReader::new(File::open("src/parse/grammar.rustpeg").expect("Opening grammar source")).lines() {
        let line = line.expect("Reading line of grammar source");

        if line.starts_with("///") {
            doc_buf.push(line);
        } else if line.starts_with("pub ") {
            doc_map.push((format!("pub fn {}", &line[4..4 + line[4..].find(" ").unwrap_or(line.len())]), doc_buf));
            doc_buf = vec![];
        } else if !doc_buf.is_empty() {
            doc_map.push((format!("fn __parse_{}", &line[0..line.find(" ").unwrap_or(line.len())]), doc_buf));
            doc_buf = vec![];
        }
    }

    let grammar_rs = PathBuf::from(format!("{}/grammar.rs", out_dir));
    peg::cargo_build("src/parse/grammar.rustpeg");
    let _ = rustfmt::run(rustfmt::Input::File(grammar_rs.clone()),
                         &rustfmt::config::Config::from_toml_path(Path::new("rustfmt.toml")).expect("constructing rustfmt::Config from rustfmt.toml"));

    fs::copy(&grammar_rs, grammar_rs.with_extension("rs.nodoc")).expect("Backing up formatted grammar.rs");
    let mut out_f = File::create(&grammar_rs).expect("Creating documented grammar.rs");
    for line in BufReader::new(File::open(grammar_rs.with_extension("rs.nodoc")).expect("Opening formatted grammar.rs")).lines() {
        let line = line.expect("Reading line of formatted grammar.rs");

        if let Some(doc) = doc_correction_map.get(&line[..]) {
            writeln!(out_f, "/// {} ", doc).expect("Adding boilerplate documentation to documented grammar.rs");
        }

        if let Some((_, doc)) = doc_map.iter().find(|doc| line.starts_with(&doc.0)) {
            for doc_line in doc {
                out_f.write_all(doc_line.as_bytes()).expect("Adding parse function documentation to documented grammar.rs");
                out_f.write_all(b"\n").expect("Writing newline to documented grammar.rs");
            }
        }

        out_f.write_all(line.as_bytes()).expect("Copying line to documented grammar.rs");
        out_f.write_all(b"\n").expect("Writing newline to documented grammar.rs");
    }
}
