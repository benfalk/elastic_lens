# Changelog

All notable changes to this project will be documented in this file.

## [0.1.8] - 2023-01-24

### Added

- Ergonomic methods to `AggResult` that allow you to directly get at
  the data the enum is wrapping.  Prior to this change you had to work
  directly from the `AggResultCollection`; which may not always be the
  case if you're attempting to pass individual results around to be
  processedd by the the code that created the request for them w/o
  passing the entire collection.

## [0.1.7] - 2023-01-21

### Fixed

- Empty `bool` query building bug is fixed.  When attempting to build
  a search if you apply an empty criteria set, such as an `AllMatch`
  it would generate a malformed query where the `bool` node had an
  empty filter.  Now criterion are checked before being applied to a
  search to prevent this going forward.

## [0.1.6] - 2023-01-20

### Added

- Support to unwrap the `MultiResponse` structure down to it's underlying
  vector of search result payloads.

## [0.1.5] - 2023-01-20

### Removed

- Requirement for `Debug` and `Clone` for types `T` in `MultiResponse`
  is no longer needed for `Client#multi_search` to return results.  Now
  if the type `T` **does** implement both/either trait then `MultiResponse`
  will also have that functionality.

## [0.1.4] - 2023-01-17

### Added

- `Cow<'static, str>` can now be coerced to a `ScalarValue` directly.
- `u8`, `u16`, `u32`, `i32`, `i16`, `i8` are now coerced to `ScalarValue`
- `f32` is now coerced to `ScalarValue`

## [0.1.3] - 2023-01-16

### Added

- `hits_mut()` and `docs_mut()` mutable iterators to search results
- `hits_take()` and `docs_take()` to extract collections from results
- Make offical adapter clone-able so the client can also be cloned

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
