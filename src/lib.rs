//! Collect an iterator into a slice.
//!
//! Rust comes with the `Iterator::collect` method for collecting an iterator's items into
//! a heap-allocated `Vec` or any other type that implements `FromIterator`, but there's
//! no way to collect items into a stack-allocated array without manually looping over the
//! iterator. This crates provides an alternative with `collect_slice` methods that
//! collect an iterator's items into a mutable slice (of a stack-allocated array or
//! otherwise.)
//!
//! The trait is automatically implemented for any type that implements `Iterator`.
//!
//! # Examples
//!
//! ```
//! use collect_slice::CollectSlice;
//!
//! let mut orig = [0; 8];
//! (0..8).map(|i| i * 2).collect_slice_checked(&mut orig[..]);
//! assert_eq!(orig, [0, 2, 4, 6, 8, 10, 12, 14]);
//!
//! let mut buf = [42; 8];
//! orig.iter()
//!     .map(|&x| x + 10)
//!     .collect_slice_checked(&mut buf[..]);
//! assert_eq!(buf, [10, 12, 14, 16, 18, 20, 22, 24]);
//! ```
//!
//! # Usage
//!
//! This crate can be used through cargo by adding it as a dependency in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! collect_slice = "^1.2.0"
//! ```
//! and importing it in the crate root:
//!
//! ```
//! extern crate collect_slice;
//! ```
//! The provided methods can then be used by importing the trait within individual
//! modules:
//!
//! ```
//! use collect_slice::CollectSlice;
//! ```

/// An iterator that can collect into a slice.
pub trait CollectSlice: Iterator {
    /// Loop through the iterator, sequentially writing items into the given slice until
    /// either the iterator runs out of items or the slice fills up.
    ///
    /// Return the number of items written.
    ///
    /// # Examples
    ///
    /// ```
    /// use collect_slice::CollectSlice;
    ///
    /// let mut buf = [0; 10];
    ///
    /// // Fill a whole slice.
    /// let count = (0..10).collect_slice(&mut buf[..]);
    /// assert_eq!(count, 10);
    /// assert_eq!(buf, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    ///
    /// // Write into a subslice
    /// let count = (10..20).collect_slice(&mut buf[5..7]);
    /// assert_eq!(count, 2);
    /// assert_eq!(buf, [0, 1, 2, 3, 4, 10, 11, 7, 8, 9]);
    ///
    /// // Only writes until iterator is exhausted.
    /// let count = (8..10).collect_slice(&mut buf[..]);
    /// assert_eq!(count, 2);
    /// assert_eq!(buf, [8, 9, 2, 3, 4, 10, 11, 7, 8, 9]);
    ///
    /// // Extra iterator items are ignored.
    /// let count = (20..40).collect_slice(&mut buf[..]);
    /// assert_eq!(count, 10);
    /// assert_eq!(buf, [20, 21, 22, 23, 24, 25, 26, 27, 28, 29]);
    /// ```
    fn collect_slice(&mut self, slice: &mut [Self::Item]) -> usize;

    /// Perform `collect_slice()` and panic if iterator yielded too few items to fill the
    /// slice.
    ///
    /// If this function succeeds, the number of items written equals the size of the
    /// given slice.
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// use collect_slice::CollectSlice;
    ///
    /// let mut buf = [0; 10];
    ///
    /// // Succeeds as long as entire slice is filled.
    /// (0..20).collect_slice_fill(&mut buf[..]);
    /// (0..5).collect_slice_fill(&mut buf[..5]);
    ///
    /// // Panics otherwise!
    /// (0..5).collect_slice_fill(&mut buf[..]);
    /// ```
    fn collect_slice_fill(&mut self, slice: &mut [Self::Item]) {
        assert_eq!(self.collect_slice(slice), slice.len());
    }

    /// Perform `collect_slice()` and panic if the slice was too small to hold all the
    /// items.
    ///
    /// Return the number of items written.
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// use collect_slice::CollectSlice;
    ///
    /// let mut buf = [0; 10];
    ///
    /// // Succeeds as long as iterator yields all its items.
    /// let count = (0..10).collect_slice_exhaust(&mut buf[..]);
    /// assert_eq!(count, 10);
    /// let count = (0..5).collect_slice_exhaust(&mut buf[..]);
    /// assert_eq!(count, 5);
    ///
    /// // Panics otherwise!
    /// (0..20).collect_slice_exhaust(&mut buf[..]);
    ///
    /// ```
    fn collect_slice_exhaust(&mut self, slice: &mut [Self::Item]) -> usize {
        let count = self.collect_slice(slice);
        assert!(self.next().is_none());
        count
    }

    /// Perform `collect_slice()` and panic if there weren't enough items to fill up
    /// the slice or the slice was too small to hold all the items.
    ///
    /// If this function succeeds, the number of items written equals the size of the
    /// given slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use collect_slice::CollectSlice;
    ///
    /// // Succeeds as long as iteration count equals slice capacity.
    /// let mut buf = [0; 10];
    /// (0..10).collect_slice_checked(&mut buf[..]);
    /// (0..5).collect_slice_checked(&mut buf[2..7]);
    /// ```
    /// ```rust,should_panic
    /// use collect_slice::CollectSlice;
    ///
    /// // Panics if iterator isn't exhausted!
    /// let mut buf = [0; 10];
    /// (0..20).collect_slice_checked(&mut buf[..]);
    /// ```
    /// ```rust,should_panic
    /// use collect_slice::CollectSlice;
    ///
    /// // Panics if slice isn't filled!
    /// let mut buf = [0; 10];
    /// (0..5).collect_slice_checked(&mut buf[..]);
    /// ```
    fn collect_slice_checked(&mut self, slice: &mut [Self::Item]) {
        assert_eq!(self.collect_slice_exhaust(slice), slice.len());
    }
}

impl<I: ?Sized> CollectSlice for I where I: Iterator {
    fn collect_slice(&mut self, slice: &mut [Self::Item]) -> usize {
        slice.iter_mut().zip(self).fold(0, |count, (dest, item)| {
            *dest = item;
            count + 1
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let mut buf = [0; 5];

        let count = (0..5).map(|i| {
            i + 1
        }).collect_slice(&mut buf[..]);

        assert_eq!(count, 5);
        assert_eq!(buf, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_under() {
        let mut buf = [0; 5];

        let count = (0..3).map(|i| {
            i + 1
        }).collect_slice(&mut buf[1..]);

        assert_eq!(count, 3);
        assert_eq!(buf, [0, 1, 2, 3, 0]);
    }

    #[test]
    fn test_over() {
        let mut buf = [0; 3];

        let mut iter = (0..5).map(|i| {
            i + 1
        });

        let count = iter.collect_slice(&mut buf[..]);

        assert_eq!(count, 3);
        assert_eq!(buf, [1, 2, 3]);

        assert_eq!(iter.next().unwrap(), 4);
        assert_eq!(iter.next().unwrap(), 5);
    }

    #[test]
    fn test_checked() {
        let mut buf = [0; 5];

        (0..5).map(|i| {
            i + 1
        }).collect_slice_checked(&mut buf[..]);

        assert_eq!(buf, [1, 2, 3, 4, 5]);
    }

    #[test]
    #[should_panic]
    fn test_checked_under() {
        let mut buf = [0; 5];

        (0..3).map(|i| {
            i + 1
        }).collect_slice_checked(&mut buf[..]);
    }

    #[test]
    #[should_panic]
    fn test_checked_over() {
        let mut buf = [0; 3];

        (0..5).map(|i| {
            i + 1
        }).collect_slice_checked(&mut buf[..]);
    }

    #[test]
    fn test_exhaust() {
        let mut buf = [0; 5];

        (0..3).map(|i| {
            i + 1
        }).collect_slice_exhaust(&mut buf[..]);

        assert_eq!(buf, [1, 2, 3, 0, 0]);

        (0..5).map(|i| {
            i + 1
        }).collect_slice_exhaust(&mut buf[..]);

        assert_eq!(buf, [1, 2, 3, 4, 5]);
    }

    #[test]
    #[should_panic]
    fn test_exhaust_over() {
        let mut buf = [0; 5];

        (0..7).map(|i| {
            i + 1
        }).collect_slice_exhaust(&mut buf[..]);
    }

    #[test]
    fn test_filled() {
        let mut buf = [0; 5];

        (0..5).map(|i| {
            i + 1
        }).collect_slice_fill(&mut buf[..]);

        assert_eq!(buf, [1, 2, 3, 4, 5]);

        (50..100).map(|i| {
            i + 1
        }).collect_slice_fill(&mut buf[..]);

        assert_eq!(buf, [51, 52, 53, 54, 55]);
    }

    #[test]
    #[should_panic]
    fn test_filled_under() {
        let mut buf = [0; 5];

        (0..3).map(|i| {
            i + 1
        }).collect_slice_fill(&mut buf[..]);
    }

    #[test]
    fn test_unsized() {
        let mut buf = [0; 5];

        let it: &mut Iterator<Item=_> = &mut (0..5).map(|i| {
            i + 1
        });
        let count = <Iterator<Item=_> as CollectSlice>::collect_slice(it, &mut buf[..]);

        assert_eq!(count, 5);
        assert_eq!(buf, [1, 2, 3, 4, 5]);
    }
}
