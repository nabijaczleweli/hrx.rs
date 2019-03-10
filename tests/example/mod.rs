use hrx::{HrxEntryData, HrxArchive};
use std::fs::{self, File};
use std::str::FromStr;
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

#[test]
fn invalid() {
    for entry in fs::read_dir("ext/hrx/example/invalid").unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            continue;
        }

        let mut body = String::new();
        File::open(path.clone()).unwrap().read_to_string(&mut body).unwrap();

        if body.contains("This HRX file is valid, but the files it contains are not.") {
            let arch = HrxArchive::from_str(&body).unwrap();

            for (k, v) in arch.entries {
                println!("{} -> {}", path.display(), k);
                if let HrxEntryData::File { body: Some(internal_body) } = v.data {
                    HrxArchive::from_str(&internal_body).unwrap_err();
                }
            }
        } else {
            HrxArchive::from_str(&body).unwrap_err();
        }
    }
}
