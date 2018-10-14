# Merging Iterator

[![Travis Build
Status](https://travis-ci.org/vbrandl/merging-iterator.svg?branch=master)](https://travis-ci.org/vbrandl/merging-iterator)
[![Crates.io](https://img.shields.io/crates/v/merging-iterator.svg)](https://crates.io/crates/merging-iterator)
[![docs.rs](https://docs.rs/merging-iterator/badge.svg)](https://docs.rs/merging-iterator/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/vbrandl/merging-iterator/blob/master/LICENSE-MIT)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](https://github.com/vbrandl/merging-iterator/blob/master/LICENSE-APACHE)

This crate implements an iterator, that takes two independent iterators and returns their elements in order, given the
two input iterators are sorted itself.

## Example

```rust
use merging_iterator::MergeIter;
let a = vec![0, 2, 4, 6, 8].into_iter();
let b = vec![1, 3, 5, 7, 9].into_iter();
let merger = MergeIter::new(a, b);
assert_eq!(
    vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    merger.collect::<Vec<_>>()
);
```

## License

`merging-iterator` is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
