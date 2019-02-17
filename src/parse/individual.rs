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
