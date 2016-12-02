# collect\_slice

Collect an iterator into a slice.

Rust comes with the `Iterator::collect` method for collecting an iterator's items into
a heap-allocated `Vec` or any other type that implements `FromIterator`, but there's
no way to collect items into a stack-allocated array without manually looping over the
iterator. This crates provides an alternative with `collect_slice` methods that
collect an iterator's items into a mutable slice (of a stack-allocated array or
otherwise.)

The trait is automatically implemented for any type that implements `Iterator`.

# Examples

```rust
use collect_slice::CollectSlice;

let mut orig = [0; 8];
(0..8).map(|i| i * 2).collect_slice_checked(&mut orig[..]);
assert_eq!(orig, [0, 2, 4, 6, 8, 10, 12, 14]);

let mut buf = [42; 8];
orig.iter()
    .map(|&x| x + 10)
    .collect_slice_checked(&mut buf[..]);
assert_eq!(buf, [10, 12, 14, 16, 18, 20, 22, 24]);
```

# Usage

This crate can be used through cargo by adding it as a dependency in `Cargo.toml`:

```toml
[dependencies]
collect_slice = "^1.1.0"
```
and importing it in the crate root:

```rust
extern crate collect_slice;
```
The provided methods can then be used by importing the trait within individual
modules:

```rust
use collect_slice::CollectSlice;
```
