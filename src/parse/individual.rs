/// Search the specified for the length of the first `boundary`
///
/// Returns `None` if no valid boundary exists.
///
/// # Examples
///
/// ```
/// # use hrx::parse::discover_first_boundary_length;
/// assert_eq!(discover_first_boundary_length("<=====>"), Some(5));
/// assert_eq!(discover_first_boundary_length("henlo\n<===> menlo"), Some(3));
///
/// assert_eq!(discover_first_boundary_length("<>"), None);
/// assert_eq!(discover_first_boundary_length("dupa"), None);
/// ```
pub fn discover_first_boundary_length<S: AsRef<str>>(in_data: S) -> Option<usize> {
    discover_first_boundary_length_impl(in_data.as_ref())
}

fn discover_first_boundary_length_impl(in_data: &str) -> Option<usize> {
    let begin = ascii_chars!('<').find(in_data)?;
    let length = ascii_chars!('>').find(&in_data[begin + 1..])?; // Searching from start of "====="s, so 0-based insdex of ">" will be their length

    if length == 0 { None } else { Some(length) }
}
