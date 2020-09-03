# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

[Unreleased]: https://github.com/Nugine/aligned-bytes/compare/v0.1.3...HEAD

## [0.1.3] - 2020-09-03

[0.1.3]: https://github.com/Nugine/aligned-bytes/compare/v0.1.2...v0.1.3

### Added

+ We can perform coercions on `AlignedBox<T>` like `Box<T>` (requires the feature `unstable`)

### Fixed

+ Fix UB which happens when creating an `AlignedBytes` with zero length
+ Yank `v0.1.0`...`v0.1.2`

## [0.1.2] - 2020-08-24

[0.1.2]: https://github.com/Nugine/aligned-bytes/compare/v0.1.1...v0.1.2

### Added

+ `AlignedBox` for aligned heap allocation

## [0.1.1] - 2020-07-16

[0.1.1]: https://github.com/Nugine/aligned-bytes/compare/v0.1.0...v0.1.1

### Added
+ `AlignedBytes` is now `Send + Sync`.
+ `AlignedBytes` is now `UnwindSafe + RefUnwindSafe` (requires the feature `std`).

## [0.1.0] - 2020-07-16

[0.1.0]: https://github.com/Nugine/aligned-bytes/tree/v0.1.0

### Added
+ `AlignedBytes` implementation.
