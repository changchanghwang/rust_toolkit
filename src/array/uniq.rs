use std::collections::HashSet;
use std::hash::Hash;

/// Removes duplicate items from an iterable while preserving the order of
/// first occurrence. Returns a `Vec` containing only the first occurrence of
/// each unique item.
///
/// # Arguments
///
/// - `items` - An iterable that produces items of type `T`
///
/// # Returns
///
/// A `Vec<T>` with duplicates removed while preserving order
///
/// # Type Parameters
///
/// - `T` - The item type. Must implement `Eq + Hash + Clone`.
///   For user-defined structs, consider `#[derive(PartialEq, Eq, Hash, Clone)]`.
///
/// # Examples
///
/// Basic types:
/// ```rust
/// use rust_toolkit::uniq;
///
/// let items = vec![1, 2, 3, 4, 5, 1, 2, 3];
/// let result = uniq(items);
/// assert_eq!(result, vec![1, 2, 3, 4, 5]);
/// ```
///
/// User-defined struct:
/// ```rust
/// use rust_toolkit::uniq;
///
/// #[derive(Clone, PartialEq, Eq, Hash, Debug)]
/// struct User { id: u32, name: &'static str }
///
/// let users = vec![
///     User { id: 1, name: "Alice" },
///     User { id: 1, name: "Alice" },
///     User { id: 2, name: "Bob" },
/// ];
/// let unique = uniq(users);
/// assert_eq!(unique.len(), 2);
/// ```
pub fn uniq<T>(items: impl IntoIterator<Item = T>) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut seen: HashSet<T> = HashSet::new();
    let mut result: Vec<T> = Vec::new();

    for item in items.into_iter() {
        if seen.insert(item.clone()) {
            result.push(item);
        }
    }

    result
}

/// Extension trait that adds the `uniq` method to any iterator.
///
/// This trait provides a convenient `uniq` method so you can call it directly
/// on any iterator to collect unique items while preserving order.
pub trait UniqExt: Iterator {
    /// Collects unique items from the iterator into a `Vec`,
    /// preserving the order of first occurrence.
    ///
    /// # Returns
    ///
    /// A `Vec<Self::Item>` with duplicates removed
    ///
    /// # Type Constraints
    ///
    /// - `Self::Item: Eq + Hash + Clone`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_toolkit::UniqExt;
    ///
    /// let items = vec![1, 2, 3, 4, 5, 1, 2, 3];
    /// let result = items.into_iter().uniq();
    /// assert_eq!(result, vec![1, 2, 3, 4, 5]);
    /// ```
    ///
    /// For user-defined structs, you may need to derive traits:
    /// ```rust
    /// use rust_toolkit::UniqExt;
    ///
    /// #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    /// struct User { id: u32, name: &'static str }
    ///
    /// let users = vec![
    ///     User { id: 1, name: "Alice" },
    ///     User { id: 1, name: "Alice" },
    ///     User { id: 2, name: "Bob" },
    /// ];
    /// let unique = users.into_iter().uniq();
    /// assert_eq!(unique.len(), 2);
    /// ```
    fn uniq(self) -> Vec<Self::Item>
    where
        Self: Sized,
        Self::Item: Eq + Hash + Clone,
    {
        uniq(self)
    }
}

impl<I: Iterator> UniqExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniq() {
        let items = vec![1, 2, 3, 4, 5, 1, 2, 3];
        let result = uniq(items);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_uniq_ext() {
        let items = vec![1, 2, 3, 4, 5, 1, 2, 3];
        let result = items.into_iter().uniq();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
}
