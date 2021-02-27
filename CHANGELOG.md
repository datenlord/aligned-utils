# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

[Unreleased]: https://github.com/datenlord/aligned-utils/compare/v1.0.2...HEAD

## [1.0.2] - 2021-02-27

[1.0.2]: https://github.com/datenlord/aligned-utils/compare/v1.0.1...v1.0.2

### Added

+ impl `From<Box<[u8]>>` for `AlignedBytes`

## [1.0.1] - 2021-01-28

[1.0.1]: https://github.com/datenlord/aligned-utils/compare/v1.0.0...v1.0.1

### Added

+ impl `Clone` for `AlignedBytes`

## [1.0.0] - 2021-01-12

[1.0.0]: https://github.com/datenlord/aligned-utils/compare/v0.2.0...v1.0.0

### Added

+ Add `Align128` ~ `Align4096`.

### Changed

+ `AlignedBytes::into_raw` takes `this: Self`

## [0.2.0] - 2020-12-31

[0.2.0]: https://github.com/datenlord/aligned-utils/compare/v0.1.5...v0.2.0

The project has been renamed to `aligned-utils`.

### Added

+ Add `AlignedBytes::new_from_slice`

### Changed

* Move `AlignedBytes` to `crate::bytes::AlignedBytes`.

### Removed

- Remove `AlignedBox`

### Fixed

+ Yank `v0.1.4`...`v0.1.5` due to a null pointer UB.

## [0.1.5] - 2020-10-18

[0.1.5]: https://github.com/datenlord/aligned-utils/compare/v0.1.4...v0.1.5

The project has been moved from `Nugine/aligned-bytes` to `datenlord/aligned-bytes`.

### Added

+ Add aligned wrappers which allow const values and stack values to be aligned.

## [0.1.4] - 2020-09-05

[0.1.4]: https://github.com/datenlord/aligned-utils/compare/v0.1.3...v0.1.4

### Fixed

+ Fix bug: `AlignedBox::new` drops ZST
+ Yank `v0.1.3`

## [0.1.3] - 2020-09-03

[0.1.3]: https://github.com/datenlord/aligned-utils/compare/v0.1.2...v0.1.3

### Added

+ We can perform coercions on `AlignedBox<T>` like `Box<T>` (requires the feature `unstable`)

### Fixed

+ Fix UB which happens when creating an `AlignedBytes` with zero length
+ Yank `v0.1.0`...`v0.1.2`

## [0.1.2] - 2020-08-24

[0.1.2]: https://github.com/datenlord/aligned-utils/compare/v0.1.1...v0.1.2

### Added

+ `AlignedBox` for aligned heap allocation

## [0.1.1] - 2020-07-16

[0.1.1]: https://github.com/datenlord/aligned-utils/compare/v0.1.0...v0.1.1

### Added
+ `AlignedBytes` is now `Send + Sync`.
+ `AlignedBytes` is now `UnwindSafe + RefUnwindSafe` (requires the feature `std`).

## [0.1.0] - 2020-07-16

[0.1.0]: https://github.com/datenlord/aligned-utils/tree/v0.1.0

### Added
+ `AlignedBytes` implementation.
