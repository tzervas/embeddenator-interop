# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.21.0] - 2026-01-25

### Added
- Full envelope compression support (Zstd, LZ4)
- Automated cbindgen C header generation
- Round-trip compression verification
- Feature flags for optional compression algorithms

### Changed
- API stabilized for FFI and language interoperability

## [0.20.0-alpha.1] - 2026-01-16

### Added
- Initial alpha release
- Python FFI bindings
- C bindings foundation
- Integration with embeddenator-vsa, embeddenator-fs, embeddenator-io
