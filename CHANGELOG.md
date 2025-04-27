# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

I'm new at this, so expect imperfection ;_; I'm trying!

## Unreleased

### Changed

+ __Breaking__: changed `read_config` parameter type to `&Path` from `&str`.

## [0.3.1] - 2024-02-24

### Changed

+ Changed some error messages to be more helpful
  + New error message if `systems` key contains a value instead of a set of labels
  + Prefixes all errors with "archive config error"

## [0.3.0] - 2024-01-23

### Changed

+ __Breaking:__ extracted `System` struct to dedicated module
  + Changes import path from `arcconfig::System` to `arcconfig::system::System`

### Documentation

+ Added inline rustdoc for `read_config` and `System` (#10)
+ Fixed incorrect dates in the changelog

## [0.2.1] - 2024-01-14

### Added

+ Added more error messages (#14)
+ Added system label to system-specific error messages (#12)

### Removed

+ Removed test function that was causing GitHub Actions to fail

### Documentation

+ Added project synposis and purpose
+ Added usage instructions as a project dependency
+ Added cargo version and license [shields](https://shields.io)
+ Added links to arcosystem projects

### Miscellaneous

+ Added basic GitHub Actions for automated testing
