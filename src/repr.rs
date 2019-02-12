use linked_hash_map::LinkedHashMap;


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
