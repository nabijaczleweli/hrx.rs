use linked_hash_map::LinkedHashMap;
use std::borrow::Cow;


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HrxArchive<'dt> {
    pub comment: Option<Cow<'dt, str>>,
    pub entries: LinkedHashMap<Cow<'dt, str>, HrxEntry<'dt>>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HrxEntry<'dt> {
    pub comment: Option<Cow<'dt, str>>,
    pub data: HrxEntryData<'dt>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum HrxEntryData<'dt> {
    File { body: Option<Cow<'dt, str>>, },
    Directory,
}
