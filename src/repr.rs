use self::super::{grammar, HrxError};
use linked_hash_map::LinkedHashMap;
use std::borrow::Borrow;
use std::str::FromStr;
use std::fmt;


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HrxArchive {
    pub comment: Option<String>,
    pub entries: LinkedHashMap<HrxPath, HrxEntry>,
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HrxPath(pub(crate) String);


impl FromStr for HrxArchive {
    type Err = HrxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = grammar::discover_first_boundary_length(s).ok_or(HrxError::NoBoundary)?;
        let parsed = grammar::archive(s, width)?;

        Ok(parsed)
    }
}

impl HrxPath {
    /// Unwraps the contained path.
    ///
    /// ```
    /// # use hrx::HrxPath;
    /// # use std::str::FromStr;
    /// let path = HrxPath::from_str("хэнло/communism.exe").unwrap();
    /// let raw = path.into_inner();
    ///
    /// assert_eq!(raw, "хэнло/communism.exe");
    /// ```
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for HrxPath {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.0)
    }
}

impl FromStr for HrxPath {
    type Err = HrxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = grammar::path(s, 0)?;

        Ok(parsed)
    }
}

impl Borrow<str> for HrxPath {
    fn borrow(&self) -> &str {
        &self.0
    }
}
