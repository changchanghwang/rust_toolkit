use std::collections::HashMap;
use std::hash::Hash;

/// Creates a HashMap from an iterable collection by using a key resolver function.
///
/// This function takes any iterable collection and transforms it into a HashMap
/// where the keys are derived from the items using the provided key resolver function.
///
/// # Arguments
///
/// * `items` - An iterable collection of items of type `T`
/// * `f` - A function that takes a reference to an item and returns a key of type `K`
///
/// # Returns
///
/// A HashMap where keys are of type `K` and values are of type `T`
///
/// # Type Parameters
///
/// * `T` - The type of items in the input collection
/// * `K` - The type of keys in the resulting HashMap (must implement Hash + Eq)
/// * `F` - The type of the key resolver function
///
/// # Examples
///
/// ```rust
/// use rust_toolkit::key_by;
///
/// let people = vec!["Alice", "Bob", "Charlie"];
/// let by_length = key_by(people, |name| name.len());
/// // Results in: {5: "Alice", 3: "Bob", 7: "Charlie"}
/// ```
pub fn key_by<T, K, F>(items: impl IntoIterator<Item = T>, key_resolver: F) -> HashMap<K, T>
where
    K: Hash + Eq,
    F: Fn(&T) -> K,
{
    items
        .into_iter()
        .map(|item| {
            // Apply the key resolver function to get the key for this item
            let key = key_resolver(&item);
            (key, item)
        })
        .collect()
}

/// Extension trait that adds the `key_by` method to any iterator.
///
/// This trait provides a convenient way to transform iterators into HashMaps
/// by calling the `key_by` method directly on the iterator.
pub trait KeyByExt: Iterator {
    /// Transforms the iterator into a HashMap using a key resolver function.
    ///
    /// This method consumes the iterator and creates a HashMap where each item
    /// becomes a value and its corresponding key is determined by the key resolver function.
    ///
    /// # Arguments
    ///
    /// * `key_resolver` - A function that takes a reference to an item and returns a key
    ///
    /// # Returns
    ///
    /// A HashMap where keys are derived from the items using the key resolver function
    ///
    /// # Type Parameters
    ///
    /// * `K` - The type of keys in the resulting HashMap (must implement Hash + Eq)
    /// * `F` - The type of the key resolver function
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_toolkit::KeyByExt;
    ///
    /// let numbers = vec![1, 2, 3, 4, 5];
    /// let by_remainder = numbers.into_iter().key_by(|&n| n % 2);
    /// // Results in: {1: 1, 0: 2} (later items with same key overwrite earlier ones)
    /// ```
    fn key_by<K, F>(self, key_resolver: F) -> HashMap<K, Self::Item>
    where
        Self: Sized,
        K: Hash + Eq,
        F: Fn(&Self::Item) -> K,
    {
        key_by(self, key_resolver)
    }
}

/// Blanket implementation of KeyByExt for all iterator types.
///
/// This implementation allows any iterator to use the `key_by` method
/// without needing to explicitly implement the trait.
impl<I: Iterator> KeyByExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct User {
        id: u32,
        name: &'static str,
    }

    fn create_users() -> Vec<User> {
        vec![
            User {
                id: 1,
                name: "Alice",
            },
            User { id: 2, name: "Bob" },
            User {
                id: 3,
                name: "Charlie",
            },
        ]
    }

    #[test]
    fn test_key_by_fn() {
        let users = create_users();
        let user_map = key_by(users, |user| user.id);
        assert_eq!(
            user_map,
            HashMap::from([
                (
                    1,
                    User {
                        id: 1,
                        name: "Alice"
                    }
                ),
                (2, User { id: 2, name: "Bob" }),
                (
                    3,
                    User {
                        id: 3,
                        name: "Charlie"
                    }
                )
            ])
        );
    }

    #[test]
    fn test_key_by_ext() {
        let users = create_users();
        let user_map = users.into_iter().key_by(|user| user.id);
        assert_eq!(
            user_map,
            HashMap::from([
                (
                    1,
                    User {
                        id: 1,
                        name: "Alice"
                    }
                ),
                (2, User { id: 2, name: "Bob" }),
                (
                    3,
                    User {
                        id: 3,
                        name: "Charlie"
                    }
                )
            ])
        );
    }
}
