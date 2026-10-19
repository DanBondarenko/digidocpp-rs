# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-10-19

Initial release.

### Added

- `digidocpp`: safe bindings to [libdigidocpp](https://github.com/open-eid/libdigidocpp). Create and validate eIDAS
  digital signatures in ASiC-E/XAdES containers.
- `libdigidocpp-sys`: low-level `cxx` bindings to libdigidocpp.
- `libdigidocpp-build`: build-script helpers that locate libdigidocpp and emit directives.
- Examples: a command-line tool and a server in `examples/`.

### Notes

- Requires libdigidocpp 4.5.0 or newer, linked dynamically.
- Requires Rust 1.88 or newer and a C++20 compiler.

[Unreleased]: https://github.com/DanBondarenko/digidocpp-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/DanBondarenko/digidocpp-rs/releases/tag/v0.1.0
