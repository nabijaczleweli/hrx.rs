use self::super::{grammar, HrxError};
use linked_hash_map::LinkedHashMap;
use std::str::FromStr;


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HrxArchive {
    pub comment: Option<String>,
    pub entries: LinkedHashMap<String, HrxEntry>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HrxEntry {
    pub comment: Option<String>,
    pub data: HrxEntryData,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum HrxEntryData {
    File { body: Option<String>, },
    Directory,
}


impl FromStr for HrxArchive {
    type Err = HrxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = grammar::discover_first_boundary_length(s).ok_or(HrxError::NoBoundary)?;
        let parsed = grammar::archive(s, width)?;

        Ok(parsed)
    }
}
