use std::collections::HashMap;
use std::hash::Hash;

/// Counts items from an iterable collection grouped by a key derived from a resolver function.
///
/// This function iterates over the input items, derives a key for each item using the provided
/// key resolver function, and returns a `HashMap` that maps each key to the number of items that
/// produced that key.
///
/// # Arguments
///
/// * `items` - An iterable collection of items of type `T`
/// * `key_resolver` - A function that takes a reference to an item and returns a key of type `K`
///
/// # Returns
///
/// A `HashMap<K, usize>` where each key corresponds to the number of items that produced it.
///
/// # Type Parameters
///
/// * `T` - The type of items in the input collection
/// * `K` - The type of keys in the resulting `HashMap` (must implement `Hash + Eq`)
///
/// # Examples
///
/// ```rust
/// use rust_toolkit::count_by;
///
/// let items = vec![1, 2, 3, 4, 5, 6];
/// let parity = count_by(items, |&n| if n % 2 == 0 { "even" } else { "odd" });
/// assert_eq!(parity, std::collections::HashMap::from([("odd", 3), ("even", 3)]));
/// ```
///
/// ```rust
/// use rust_toolkit::count_by;
///
/// let words = vec!["apple", "banana", "apricot", "blueberry"];
/// let by_first_letter = count_by(words, |w| w.chars().next().unwrap());
/// assert_eq!(by_first_letter, std::collections::HashMap::from([('a', 2usize), ('b', 2usize)]));
/// ```
pub fn count_by<T, K>(
    items: impl IntoIterator<Item = T>,
    key_resolver: impl Fn(&T) -> K,
) -> HashMap<K, usize>
where
    K: Hash + Eq,
{
    let mut map = HashMap::new();
    for item in items {
        // Derive the key for this item and increment the corresponding counter
        let key = key_resolver(&item);
        map.entry(key).and_modify(|count| *count += 1).or_insert(1);
    }
    map
}

/// Extension trait that adds the `count_by` method to any iterator.
///
/// This trait provides a convenient way to count items by a derived key directly from an iterator.
pub trait CountByExt: Iterator {
    /// Counts the iterator items by a key derived from a resolver function.
    ///
    /// This method consumes the iterator and returns a `HashMap` mapping each derived key
    /// to the number of items that produced that key.
    ///
    /// # Arguments
    ///
    /// * `key_resolver` - A function that takes a reference to an item and returns a key
    ///
    /// # Returns
    ///
    /// A `HashMap` where keys are derived from the items using the key resolver function and
    /// values are the corresponding counts.
    ///
    /// # Type Parameters
    ///
    /// * `K` - The type of keys in the resulting `HashMap` (must implement `Hash + Eq`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_toolkit::CountByExt;
    ///
    /// let items = vec![1, 2, 3, 4, 5];
    /// let counts = items.into_iter().count_by(|&n| if n % 2 == 0 { "even" } else { "odd" });
    /// assert_eq!(counts, std::collections::HashMap::from([("odd", 3), ("even", 2)]));
    /// ```
    fn count_by<K>(self, key_resolver: impl Fn(&Self::Item) -> K) -> HashMap<K, usize>
    where
        Self: Sized,
        K: Hash + Eq,
    {
        count_by(self, key_resolver)
    }
}

/// Blanket implementation of `CountByExt` for all iterator types.
impl<I: Iterator> CountByExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_by_fn() {
        let items = vec![1, 2, 3, 4, 5];
        let result = count_by(items, |&item| if item % 2 == 0 { "even" } else { "odd" });
        assert_eq!(result, HashMap::from([("odd", 3), ("even", 2)]));
    }

    #[test]
    fn test_count_by_ext() {
        let items = vec![1, 2, 3, 4, 5];
        let result = items
            .into_iter()
            .count_by(|&item| if item % 2 == 0 { "even" } else { "odd" });
        assert_eq!(result, HashMap::from([("odd", 3), ("even", 2)]));
    }
}
