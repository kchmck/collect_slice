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

let mut orig = [0; 96];
(0..96).map(|i| i * 2).collect_slice_checked(&mut orig[..]);

let mut buf = [0.0; 96];
orig.iter()
    .map(|&x| x as f32 * std::f32::consts::PI)
    .collect_slice_checked(&mut buf[..]);
```


# Usage

This crate can be used through cargo by adding it as a dependency in `Cargo.toml`:

```toml
[dependencies]
collect_slice = "1.0.0"
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
