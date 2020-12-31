# aligned-utils

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Docs][docs-badge]][docs-url]
![CI][ci-badge]

[crates-badge]: https://img.shields.io/crates/v/aligned-utils.svg
[crates-url]: https://crates.io/crates/aligned-utils
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[docs-badge]: https://docs.rs/aligned-utils/badge.svg
[docs-url]: https://docs.rs/aligned-utils/
[ci-badge]: https://github.com/datenlord/aligned-utils/workflows/CI/badge.svg

Common utilities to work with aligned values and allocation.

## Example

```rust
use aligned_utils::stack::Align8;
let mut arr = Align8([1, 2, 3]);
let bytes: &[u8] = &*arr;
```

```rust
use aligned_utils::bytes::AlignedBytes;  // with feature "alloc"
let mut bytes = AlignedBytes::new_zeroed(1024, 8);
let buf: &mut [u8] = &mut *bytes;
```
