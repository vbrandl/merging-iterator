# Merging Iterator

![Rust](https://github.com/vbrandl/merging-iterator/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/merging-iterator.svg)](https://crates.io/crates/merging-iterator)
[![docs.rs](https://docs.rs/merging-iterator/badge.svg)](https://docs.rs/merging-iterator/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/vbrandl/merging-iterator/blob/master/LICENSE-MIT)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](https://github.com/vbrandl/merging-iterator/blob/master/LICENSE-APACHE)

This crate implements an iterator, that takes two independent iterators and returns their elements in order, given the
two input iterators are sorted itself.

**Important note**: This iterator only works if the input iterators are already sorted since only the respective `next`
elements of each iterator are compared. There are no checks in place to validate this property.

## Example

```rust
extern crate merging_iterator;

use merging_iterator::MergeIter;
let a = vec![0, 2, 4, 6, 8];
let b = vec![1, 3, 5, 7, 9];
let merger = MergeIter::new(a, b);
assert_eq!(
    vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    merger.collect::<Vec<_>>()
);
```

You can also merge more than two sorted iterators like this:

```rust
extern crate merging_iterator;

use merging_iterator::MergeIter;
let a = vec![1, 4, 7];
let b = vec![2, 5, 8];
let c = vec![3, 6, 9];
let merger = MergeIter::new(
    MergeIter::new(a, b),
    c
);
assert_eq!(
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    merger.collect::<Vec<_>>()
);
```

## License

`merging-iterator` is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
