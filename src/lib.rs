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
//! collect_slice = "1.0.0"
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
    /// Loop through the iterator, writing items into the given slice until either the
    /// iterator runs out of items or the slice fills up. Return the number of items
    /// written.
    fn collect_slice(&mut self, slice: &mut [Self::Item]) -> usize;

    /// Perform `collect_slice()` and panic if there weren't enough items to fill up
    /// the buffer or the buffer was too small to hold all the items.
    fn collect_slice_checked(&mut self, slice: &mut [Self::Item]) {
        assert!(self.collect_slice(slice) == slice.len());
        assert!(self.next().is_none());
    }
}

impl<T, I: Iterator<Item = T>> CollectSlice for I {
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

        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);
        assert_eq!(buf[3], 4);
        assert_eq!(buf[4], 5);
    }

    #[test]
    fn test_under() {
        let mut buf = [0; 5];

        let count = (0..3).map(|i| {
            i + 1
        }).collect_slice(&mut buf[1..]);

        assert_eq!(count, 3);

        assert_eq!(buf[0], 0);
        assert_eq!(buf[1], 1);
        assert_eq!(buf[2], 2);
        assert_eq!(buf[3], 3);
        assert_eq!(buf[4], 0);
    }

    #[test]
    fn test_over() {
        let mut buf = [0; 3];

        let mut iter = (0..5).map(|i| {
            i + 1
        });

        let count = iter.collect_slice(&mut buf[..]);

        assert_eq!(count, 3);

        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);

        assert_eq!(iter.next().unwrap(), 4);
        assert_eq!(iter.next().unwrap(), 5);
    }

    #[test]
    fn test_checked() {
        let mut buf = [0; 5];

        (0..5).map(|i| {
            i + 1
        }).collect_slice_checked(&mut buf[..]);
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
}
