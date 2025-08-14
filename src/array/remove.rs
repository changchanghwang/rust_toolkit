/// Splits an iterable into kept and removed items based on a predicate.
///
/// This function consumes the input iterable and evaluates each item with the
/// provided predicate. Items for which the predicate returns `true` are
/// considered "removed"; the rest are "kept".
///
/// Returns a tuple `(kept, removed)` where both are `Vec<T>`.
///
/// # Arguments
///
/// - `items` - An iterable producing items of type `T`
/// - `should_remove_element` - Predicate determining which items to remove
///
/// # Examples
///
/// ```rust
/// use rust_toolkit::remove;
///
/// let items = vec![1, 2, 3, 4, 5];
/// let (kept, removed) = remove(items, |&n| n % 2 == 0);
/// assert_eq!(kept, vec![1, 3, 5]);
/// assert_eq!(removed, vec![2, 4]);
/// ```
pub fn remove<T>(
    items: impl IntoIterator<Item = T>,
    should_remove_element: impl Fn(&T) -> bool,
) -> (Vec<T>, Vec<T>) {
    // Partition moves items into either the first or second vector.
    let (removed, kept): (Vec<T>, Vec<T>) = items
        .into_iter()
        .partition(|item| should_remove_element(item));
    (kept, removed)
}

/// Extension trait that adds the `remove` method to any iterator.
pub trait RemoveExt: Iterator {
    /// Splits the iterator items into kept and removed based on a predicate.
    ///
    /// Returns `(kept, removed)`.
    fn remove(
        self,
        should_remove_element: impl Fn(&Self::Item) -> bool,
    ) -> (Vec<Self::Item>, Vec<Self::Item>)
    where
        Self: Sized,
    {
        remove(self, should_remove_element)
    }
}

impl<I: Iterator> RemoveExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_fn_splits_correctly() {
        let items = vec![1, 2, 3, 4, 5];
        let (kept, removed) = remove(items, |&n| n % 2 == 0);
        assert_eq!(kept, vec![1, 3, 5]);
        assert_eq!(removed, vec![2, 4]);
    }

    #[test]
    fn test_remove_ext_on_iterator() {
        let items = vec!["a", "bb", "ccc", "dddd"];
        let (kept, removed) = items.into_iter().remove(|s| s.len() % 2 == 0);
        assert_eq!(kept, vec!["a", "ccc"]);
        assert_eq!(removed, vec!["bb", "dddd"]);
    }

    #[test]
    fn test_remove_empty() {
        let items: Vec<i32> = vec![];
        let (kept, removed) = remove(items, |_n| true);
        assert!(kept.is_empty());
        assert!(removed.is_empty());
    }
}
