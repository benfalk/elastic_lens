# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2] - 2023-01-16

### Removed

- Requirement for `Debug` and `Clone` for types `T` in `SearchResults`
  is no longer required in order for `Client#search` to return results.
  Now if the type `T` **does** implement both/either trait then
  `SearchResults` will also have that functionality.

## [0.1.1] - 2023-01-15

### Changed

- Spelling references from `offical` to `official`

## [0.1.0] - 2023-01-14

### Added

- Initial official release and published to `crates.io`
