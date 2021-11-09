use std::collections::btree_map::{BTreeMap, Entry as BTreeMapEntry};
use self::super::super::{HrxEntryData, HrxEntry, HrxError, HrxPath};
use linked_hash_map::LinkedHashMap;
use std::num::NonZeroUsize;
use memchr::memchr;


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
    let begin = memchr(b'<', in_data.as_bytes())?;
    let length = memchr(b'>', in_data[begin + 1..].as_bytes())?; // Searching from start of "====="s, so 0-based index of ">" will be their length

    NonZeroUsize::new(length)
}


/// Convert a collexion of `(path, entry)` pairs into a `path -> entry` map, erroring on any duplicates and file-as-dir usages.
///
/// # Examples
///
/// Dupe:
///
/// ```
/// # use hrx::{HrxEntryData, HrxEntry, HrxError, HrxPath};
/// # use hrx::parse::reduce_raw_entries_and_validate_directory_tree;
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
/// assert_eq!(reduce_raw_entries_and_validate_directory_tree(source_material.clone()),
///            Ok(source_material.iter().cloned().collect()));
///
/// // Introducing a dupe, now both files have the same paths
/// source_material[1].0 = source_material[0].0.clone();
///
/// assert_eq!(reduce_raw_entries_and_validate_directory_tree(source_material.clone()),
///            Err(HrxError::DuplicateEntry(source_material[0].0.to_string())));
/// // i.e.
/// assert_eq!(reduce_raw_entries_and_validate_directory_tree(source_material.clone()),
///            Err(HrxError::DuplicateEntry("file1.txt".to_string())));
/// ```
///
/// File as directory:
///
/// ```
/// # use hrx::{HrxEntryData, HrxEntry, HrxError, HrxPath};
/// # use hrx::parse::reduce_raw_entries_and_validate_directory_tree;
/// # use std::num::NonZeroUsize;
/// let mut source_material = vec![("file1.txt".parse().unwrap(),
///                                 HrxEntry {
///                                     comment: None,
///                                     data: HrxEntryData::File {
///                                         body: Some("First file's contents".to_string())
///                                     }
///                                 }),
///                                ("file1.txt/subfile.txt".parse().unwrap(),
///                                 HrxEntry {
///                                     comment: None,
///                                     data: HrxEntryData::File {
///                                         body: Some("Second file's contents, using the first file as directory".to_string())
///                                     }
///                                 })];
///
/// assert_eq!(reduce_raw_entries_and_validate_directory_tree(source_material.clone()),
///            Err(HrxError::FileAsDirectory(source_material[0].0.to_string(), source_material[1].0.to_string())));
/// // i.e.
/// assert_eq!(reduce_raw_entries_and_validate_directory_tree(source_material.clone()),
///            Err(HrxError::FileAsDirectory("file1.txt".to_string(), "file1.txt/subfile.txt".to_string())));
/// ```
pub fn reduce_raw_entries_and_validate_directory_tree<Ii: IntoIterator<Item = (HrxPath, HrxEntry)>>(iter: Ii)
                                                                                                    -> Result<LinkedHashMap<HrxPath, HrxEntry>, HrxError> {
    let iter = iter.into_iter();
    let mut map = LinkedHashMap::with_capacity(iter.size_hint().0);
    let mut paths = BTreeMap::new();

    for (k, v) in iter {
        reduce_raw_entry_and_validate_its_directory_tree(k, v, &mut map, &mut paths)?;
    }

    Ok(map)
}

fn reduce_raw_entry_and_validate_its_directory_tree(k: HrxPath, v: HrxEntry, map: &mut LinkedHashMap<HrxPath, HrxEntry>, paths: &mut BTreeMap<String, bool>)
                                                    -> Result<(), HrxError> {
    for (slash_i, _) in k.0.match_indices('/') {
        match paths.entry(k.0[0..slash_i].to_string()) {
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

    match paths.entry(k.0) {
        BTreeMapEntry::Vacant(ve) => {
            let is_dir = v.data == HrxEntryData::Directory;

            map.insert(HrxPath(ve.key().clone()), v);

            ve.insert(is_dir);
        }
        BTreeMapEntry::Occupied(oe) => {
            return Err(HrxError::DuplicateEntry(oe.remove_entry().0));
        }
    }

    Ok(())
}
