# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `digidocpp`: most of the API now requires the thread-exclusive library handle `Library`. 
  This handle is `!Send` and `!Sync`. This ensures only a single thread can use the library at a time.
  The library handle can be obtained via `acquire`/`try_acquire` or `Library<_>::acquire`/`Library<_>::try_acquire`.
  The library can be in one of several process-wide states: pre-initialization (`Library<Uninitialized>`), 
  initialized (`Library<Initialized>`), or terminated.
  The state transitions require explicit method calls (e.g. `Library<Uninitialized>::initialize`, 
  `Library<Initialized>::terminate`) and are permanent on the process level.
- `digidocpp`: the `Access` enum represents the two states which consumers may find the library in (when 
  calling `acquire`/`try_acquire`).
- `digidocpp`: the `AccessError` enum represents the ways in which accessing the library may fail.

### Changed

- **Breaking:** Free functions relating to the library lifecycle and information are now methods of `Library`.
  This includes: `initialize`, `initialize_with_app_info`, `initialize_with_user_agent`, `init_conf`, `terminate`, 
  `version`, `app_info` and `user_agent`. 
- **Breaking:** Constructors now generally take a library handle as the first parameter. 
  These now require `&Library<Uninitialized>`: `Conf::new`, `Conf::from_file`, `Conf::from_file_with_schema`, 
  `Conf::with_schema`.
  These now require `&Library<Initialized>`: `Container::open`, `Container::create`, 
  `Container::open_requiring_online_validation_consent`, `ExternalPartialSigner::new`, `Pkcs12Signer::new`, 
  `Pkcs11Signer::new`, `WinSigner::new`.
- **Breaking:** `Container` and signers now carry a lifetime that borrows `&Library<Initialized>`. 
- **Breaking:** the library can now only be initialized and terminated only once per process. This is enforced at type 
  level.

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
