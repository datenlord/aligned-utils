# aligned-bytes

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Docs][docs-badge]][docs-url]
![CI][ci-badge]

[crates-badge]: https://img.shields.io/crates/v/aligned-bytes.svg
[crates-url]: https://crates.io/crates/aligned-bytes
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[docs-badge]: https://docs.rs/aligned-bytes/badge.svg
[docs-url]: https://docs.rs/aligned-bytes/
[ci-badge]: https://github.com/datenlord/aligned-bytes/workflows/CI/badge.svg

A continuous fixed-length byte array with a specified alignment.

## Example
```rust
use aligned_bytes::AlignedBytes;
let mut bytes = AlignedBytes::new_zeroed(1024, 8);
let buf: &mut [u8] = &mut *bytes;
```
