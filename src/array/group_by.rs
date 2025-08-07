use std::collections::HashMap;
use std::hash::Hash;

/// Groups items from an iterable collection into a HashMap based on a key resolver function.
///
/// This function takes any iterable collection and groups the items by keys derived
/// from the items using the provided key resolver function. Items with the same key
/// will be collected into vectors.
///
/// # Arguments
///
/// * `items` - An iterable collection of items of type `T`
/// * `key_resolver` - A function that takes a reference to an item and returns a key of type `K`
///
/// # Returns
///
/// A HashMap where keys are of type `K` and values are vectors of items of type `T`
/// that share the same key.
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
/// use rust_toolkit::group_by;
///
/// let people = vec!["Alice", "Bob", "Charlie", "Anna"];
/// let by_first_letter = group_by(people, |name| name.chars().next().unwrap());
/// // Results in: {'A': ["Alice", "Anna"], 'B': ["Bob"], 'C': ["Charlie"]}
/// ```
///
/// ```rust
/// use rust_toolkit::group_by;
///
/// let numbers = vec![1, 2, 3, 4, 5, 6];
/// let by_even_odd = group_by(numbers, |&n| n % 2);
/// // Results in: {1: [1, 3, 5], 0: [2, 4, 6]}
/// ```
pub fn group_by<T, K, F>(items: impl IntoIterator<Item = T>, key_resolver: F) -> HashMap<K, Vec<T>>
where
    K: Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut map = HashMap::new();
    for item in items {
        // Apply the key resolver function to determine which group this item belongs to
        let key = key_resolver(&item);
        // Insert the item into the appropriate group, creating a new vector if needed
        map.entry(key).or_insert(vec![]).push(item);
    }
    map
}

/// Extension trait that adds the `group_by` method to any iterator.
///
/// This trait provides a convenient way to group iterator items into HashMaps
/// by calling the `group_by` method directly on the iterator.
pub trait GroupByExt: Iterator {
    /// Groups the iterator items into a HashMap using a key resolver function.
    ///
    /// This method consumes the iterator and creates a HashMap where items with the same
    /// key (as determined by the key resolver function) are collected into vectors.
    ///
    /// # Arguments
    ///
    /// * `key_resolver` - A function that takes a reference to an item and returns a key
    ///
    /// # Returns
    ///
    /// A HashMap where keys are derived from the items using the key resolver function
    /// and values are vectors of items that share the same key.
    ///
    /// # Type Parameters
    ///
    /// * `K` - The type of keys in the resulting HashMap (must implement Hash + Eq)
    /// * `F` - The type of the key resolver function
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_toolkit::GroupByExt;
    ///
    /// let words = vec!["apple", "banana", "apricot", "blueberry"];
    /// let by_first_letter = words.into_iter().group_by(|word| word.chars().next().unwrap());
    /// // Results in: {'a': ["apple", "apricot"], 'b': ["banana", "blueberry"]}
    /// ```
    ///
    /// ```rust
    /// use rust_toolkit::GroupByExt;
    ///
    /// let numbers = vec![1, 2, 3, 4, 5, 6];
    /// let by_remainder = numbers.into_iter().group_by(|&n| n % 3);
    /// // Results in: {1: [1, 4], 2: [2, 5], 0: [3, 6]}
    /// ```
    fn group_by<K, F>(self, key_resolver: F) -> HashMap<K, Vec<Self::Item>>
    where
        Self: Sized,
        K: Hash + Eq,
        F: Fn(&Self::Item) -> K,
    {
        let mut map = HashMap::new();
        for item in self {
            // Apply the key resolver function to determine which group this item belongs to
            let key = key_resolver(&item);
            // Insert the item into the appropriate group, creating a new vector if needed
            map.entry(key).or_insert(vec![]).push(item);
        }
        map
    }
}

/// Blanket implementation of GroupByExt for all iterator types.
///
/// This implementation allows any iterator to use the `group_by` method
/// without needing to explicitly implement the trait.
impl<I: Iterator> GroupByExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct User {
        id: u32,
        name: &'static str,
        age: u32,
    }

    fn create_users() -> Vec<User> {
        vec![
            User {
                id: 1,
                name: "Alice",
                age: 20,
            },
            User {
                id: 2,
                name: "Bob",
                age: 20,
            },
            User {
                id: 3,
                name: "Charlie",
                age: 30,
            },
        ]
    }

    #[test]
    fn test_group_by_fn() {
        let users = create_users();
        let user_group = group_by(users, |user| user.age);

        assert_eq!(
            user_group,
            HashMap::from([
                (
                    20,
                    vec![
                        User {
                            id: 1,
                            name: "Alice",
                            age: 20
                        },
                        User {
                            id: 2,
                            name: "Bob",
                            age: 20
                        },
                    ]
                ),
                (
                    30,
                    vec![User {
                        id: 3,
                        name: "Charlie",
                        age: 30
                    }]
                ),
            ])
        );
    }

    #[test]
    fn test_group_by_ext() {
        let users = create_users();
        let user_group = users.into_iter().group_by(|user| user.age);

        assert_eq!(
            user_group,
            HashMap::from([
                (
                    20,
                    vec![
                        User {
                            id: 1,
                            name: "Alice",
                            age: 20
                        },
                        User {
                            id: 2,
                            name: "Bob",
                            age: 20
                        },
                    ]
                ),
                (
                    30,
                    vec![User {
                        id: 3,
                        name: "Charlie",
                        age: 30
                    }]
                ),
            ])
        );
    }

    #[test]
    fn test_group_by_with_strings() {
        let words = vec!["apple", "banana", "apricot", "blueberry"];
        let grouped = group_by(words, |word| word.len());

        assert_eq!(
            grouped,
            HashMap::from([
                (5, vec!["apple"]),
                (6, vec!["banana"]),
                (7, vec!["apricot"]),
                (9, vec!["blueberry"]),
            ])
        );
    }

    #[test]
    fn test_group_by_with_numbers() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let grouped = numbers.into_iter().group_by(|&n| n % 3);

        assert_eq!(
            grouped,
            HashMap::from([(0, vec![3, 6, 9]), (1, vec![1, 4, 7]), (2, vec![2, 5, 8]),])
        );
    }

    #[test]
    fn test_group_by_empty_collection() {
        let empty_vec: Vec<i32> = vec![];
        let grouped = group_by(empty_vec, |&x| x);
        assert_eq!(grouped, HashMap::new());
    }

    #[test]
    fn test_group_by_single_item() {
        let single_item = vec![42];
        let grouped = single_item.into_iter().group_by(|&x| x % 2);
        assert_eq!(grouped, HashMap::from([(0, vec![42])]));
    }

    #[test]
    fn test_group_by_all_same_key() {
        let items = vec![1, 3, 5, 7, 9];
        let grouped = group_by(items, |&x| x % 2);
        assert_eq!(grouped, HashMap::from([(1, vec![1, 3, 5, 7, 9])]));
    }

    #[test]
    fn test_group_by_with_char_keys() {
        let words = vec!["apple", "banana", "apricot", "blueberry"];
        let grouped = words
            .into_iter()
            .group_by(|word| word.chars().next().unwrap());

        assert_eq!(
            grouped,
            HashMap::from([
                ('a', vec!["apple", "apricot"]),
                ('b', vec!["banana", "blueberry"]),
            ])
        );
    }
}
