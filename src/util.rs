//! Module containing various utility functions.


use std::num::NonZeroUsize;
use std::iter;


/// Generate a boundary string of the specified length.
///
/// Starts with a newline.
///
/// # Examples
///
/// ```
/// # use hrx::util::boundary_str;
/// # use std::num::NonZeroUsize;
/// assert_eq!(boundary_str(NonZeroUsize::new(3).unwrap()), "\n<===>");
/// ```
pub fn boundary_str(of_length: NonZeroUsize) -> String {
    let mut res = String::with_capacity(2 + of_length.get() + 1);

    res.push('\n');
    res.push('<');
    res.extend(iter::repeat('=').take(of_length.get()));
    res.push('>');

    res
}
