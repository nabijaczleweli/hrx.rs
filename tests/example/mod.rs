use std::fs::{self, File};
use std::str::FromStr;
use hrx::HrxArchive;
use std::io::Read;


#[test]
fn valid() {
    for entry in fs::read_dir("ext/hrx/example").unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            continue;
        }

        let mut body = String::new();
        File::open(path).unwrap().read_to_string(&mut body).unwrap();

        HrxArchive::from_str(&body).unwrap();
    }
}

// TODO: uncomment when dir tree validation is implemented
/*
#[test]
fn invalid() {
    for entry in fs::read_dir("ext/hrx/example/invalid").unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            continue;
        }

        let mut body = String::new();
        File::open(path).unwrap().read_to_string(&mut body).unwrap();

        HrxArchive::from_str(&body).unwrap_err();
    }
}
*/
