/// Splits an iterable collection into consecutive chunks of at most `size` items, preserving order.
///
/// The last chunk may contain fewer than `size` items if there are not enough elements remaining.
///
/// # Arguments
///
/// * `items` - The input iterable to split
/// * `size` - The maximum size of each chunk (must be greater than 0)
///
/// # Returns
///
/// A `Vec<Vec<T>>` where each inner vector is a chunk that preserves input order
///
/// # Type Parameters
///
/// * `T` - The element type
///
/// # Examples
///
/// ```rust
/// use rust_toolkit::chunk;
///
/// let items = vec![1, 2, 3, 4, 5];
/// let chunks = chunk(items, 3);
/// assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5]]);
/// ```
///
/// # Panics
///
/// Panics if `size` is 0.
pub fn chunk<T>(items: impl IntoIterator<Item = T>, size: usize) -> Vec<Vec<T>> {
    assert!(size > 0, "size must be greater than 0");

    let mut iter = items.into_iter();
    let mut chunks: Vec<Vec<T>> = Vec::new();

    loop {
        let part: Vec<T> = iter.by_ref().take(size).collect();
        if part.is_empty() {
            break;
        }
        chunks.push(part);
    }

    chunks
}

/// Extension trait that adds the `chunk` method to any iterator.
///
/// This trait provides a convenient way to split an iterator into consecutive
/// fixed-size blocks.
pub trait ChunkExt: Iterator {
    /// Splits the iterator into consecutive chunks of at most `size` items.
    ///
    /// This method consumes the iterator and returns a `Vec<Vec<Self::Item>>`
    /// that preserves input order.
    ///
    /// # Arguments
    ///
    /// * `size` - The maximum size of each chunk (must be greater than 0)
    ///
    /// # Returns
    ///
    /// A vector of chunks whose sizes are at most `size`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_toolkit::ChunkExt;
    ///
    /// let items = vec![1, 2, 3, 4, 5];
    /// let chunks = items.into_iter().chunk(2);
    /// assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5]]);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `size` is 0.
    fn chunk(self, size: usize) -> Vec<Vec<Self::Item>>
    where
        Self: Sized,
    {
        assert!(size > 0, "size must be greater than 0");

        let mut chunks: Vec<Vec<Self::Item>> = Vec::new();

        let mut iter = self.into_iter();

        loop {
            let part: Vec<Self::Item> = iter.by_ref().take(size).collect();
            if part.is_empty() {
                break;
            }
            chunks.push(part);
        }

        chunks
    }
}

/// Blanket implementation of `ChunkExt` for all iterator types.
impl<I: Iterator> ChunkExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk() {
        let items = vec![1, 2, 3, 4, 5];
        let chunks = chunk(items, 3);
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5]]);
    }

    #[test]
    fn test_chunk_ext() {
        let items = vec![1, 2, 3, 4, 5];
        let chunks = items.into_iter().chunk(3);
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5]]);
    }

    #[test]
    fn should_panic_with_zero_size() {
        let items = vec![1, 2, 3, 4, 5];

        let result = std::panic::catch_unwind(|| chunk(items, 0));
        assert!(result.is_err());
    }

    #[test]
    fn should_panic_with_zero_size_ext() {
        let items = vec![1, 2, 3, 4, 5];

        let result = std::panic::catch_unwind(|| items.into_iter().chunk(0));
        assert!(result.is_err());
    }
}
