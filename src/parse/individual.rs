use std::collections::btree_map::{BTreeMap, Entry as BTreeMapEntry};
use self::super::super::{HrxEntryData, HrxEntry, HrxError, HrxPath};
use linked_hash_map::LinkedHashMap;
use std::num::NonZeroUsize;


/// Search the specified for the length of the first `boundary`.
///
/// Returns `None` if no valid boundary exists.
///
/// # Examples
///
/// ```
/// # use hrx::parse::discover_first_boundary_length;
/// # use std::num::NonZeroUsize;
/// assert_eq!(discover_first_boundary_length("<=====>"), NonZeroUsize::new(5));
/// assert_eq!(discover_first_boundary_length("henlo\n<===> menlo"), NonZeroUsize::new(3));
///
/// assert_eq!(discover_first_boundary_length("<>"), None);
/// assert_eq!(discover_first_boundary_length("коммунизм"), None);
/// ```
pub fn discover_first_boundary_length<S: AsRef<str>>(in_data: S) -> Option<NonZeroUsize> {
    discover_first_boundary_length_impl(in_data.as_ref())
}

fn discover_first_boundary_length_impl(in_data: &str) -> Option<NonZeroUsize> {
    let begin = ascii_chars!('<').find(in_data)?;
    let length = ascii_chars!('>').find(&in_data[begin + 1..])?; // Searching from start of "====="s, so 0-based insdex of ">" will be their length

    NonZeroUsize::new(length)
}


/// Convert a collexion of `(path, entry)` pairs into a `path -> entry` map, erroring on any duplicates.
///
/// Mostly the unrolled expansion of `iter.into_iter().collect()` but with checks.
///
/// # Examples
///
/// ```
/// # use hrx::{HrxEntryData, HrxEntry, HrxError, HrxPath};
/// # use hrx::parse::reduce_raw_entries;
/// # use std::num::NonZeroUsize;
/// let mut source_material = vec![("file1.txt".parse().unwrap(),
///                                 HrxEntry {
///                                     comment: None,
///                                     data: HrxEntryData::File {
///                                         body: Some("First file's contents".to_string())
///                                     }
///                                 }),
///                                ("file2.txt".parse().unwrap(),
///                                 HrxEntry {
///                                     comment: None,
///                                     data: HrxEntryData::File {
///                                         body: Some("Second file's contents".to_string())
///                                     }
///                                 })];
///
/// // The no-dupe case
/// assert_eq!(reduce_raw_entries(source_material.clone()),
///            Ok(source_material.iter().cloned().collect()));
///
/// // Introducing a dupe, now both files have the same paths
/// source_material[1].0 = source_material[0].0.clone();
///
/// assert_eq!(reduce_raw_entries(source_material.clone()),
///            Err(HrxError::DuplicateEntry(source_material[0].0.to_string(),
///                                         source_material[0].1.clone(),
///                                         source_material[1].1.clone())));
/// // i.e.
/// assert_eq!(reduce_raw_entries(source_material.clone()),
///            Err(HrxError::DuplicateEntry("file1.txt".to_string(),
///                                         HrxEntry {
///                                             comment: None,
///                                             data: HrxEntryData::File {
///                                                 body:
///                                                     Some("First file's contents".to_string())
///                                             }
///                                         },
///                                         HrxEntry {
///                                             comment: None,
///                                             data: HrxEntryData::File {
///                                                 body:
///                                                     Some("Second file's contents".to_string())
///                                             }
///                                         })));
/// ```
pub fn reduce_raw_entries<Ii: IntoIterator<Item = (HrxPath, HrxEntry)>>(iter: Ii) -> Result<LinkedHashMap<HrxPath, HrxEntry>, HrxError> {
    let iter = iter.into_iter();
    let mut map = LinkedHashMap::with_capacity(iter.size_hint().0);

    for (k, v) in iter {
        reduce_raw_entry(k, v, &mut map)?;
    }

    Ok(map)
}

fn reduce_raw_entry(k: HrxPath, v: HrxEntry, map: &mut LinkedHashMap<HrxPath, HrxEntry>) -> Result<(), HrxError> {
    if let Some(prev) = map.insert(k, v) {
        let (path, new) = map.pop_back().unwrap();
        return Err(HrxError::DuplicateEntry(path.into_inner(), prev, new));
    }

    Ok(())
}


pub fn validate_directory_tree<'e, I: Iterator<Item = (&'e HrxPath, &'e HrxEntry)>>(iter: I) -> Result<(), HrxError> {
    let mut paths = BTreeMap::new();

    for (k, v) in iter {
        let is_dir = v.data == HrxEntryData::Directory;

        println!("({}, _)", k.0);
        for (slash_i, _) in k.0.match_indices('/') {
            println!("{}", &k.0[0..slash_i]);
            match paths.entry(&k.0[0..slash_i]) {
                BTreeMapEntry::Vacant(ve) => {
                    ve.insert(true);
                }
                BTreeMapEntry::Occupied(oe) => {
                    if !oe.get() {
                        return Err(HrxError::FileAsDirectory(oe.key().to_string(), k.0.clone()));
                    }
                }
            }
        }

        match paths.entry(&k.0[..]) {
            BTreeMapEntry::Vacant(ve) => {
                ve.insert(is_dir);
            }
            BTreeMapEntry::Occupied(oe) => {
                // dupe
                unimplemented!();
            }
        }
    }

    Ok(())
}
