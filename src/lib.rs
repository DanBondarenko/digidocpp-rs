//! Safe bindings to [libdigidocpp]: create and validate eIDAS digital signatures in ASiC-E/XAdES
//! containers.
//!
//! All major workflows of libdigidocpp are exposed to Rust. This includes:
//! - Opening existing containers and validating the digital signatures or timestamps contained
//!   therein.
//! - Creating containers from files and adding digital signatures using either [PKCS #11],
//!   [PKCS #12], Windows CryptoAPI, or an external signer (e.g. mobile ID).
//! - Extending the
//!   [validity evidence](https://open-eid.github.io/libdigidocpp/manual.html#extend-validity)
//!   of existing containers.
//! - Extracting files from containers, saving to files as well as writing to
//!   [`Write`](std::io::Write) implementations.
//!
//! Low-level bindings to libdigidocpp are provided in the [libdigidocpp-sys] crate. Valuable
//! information can also be found in the upstream [libdigidocpp manual].
//!
//! libdigidocpp is the C++ library for creating and validating digital signatures maintained by
//! the Estonian Information System Authority (RIA). This crate is unofficial and not affiliated
//! with RIA.
//!
//! # Code examples
//!
//! ## Opening and inspecting a container
//!
//! You can open a container with [`Container::open`]. Signatures are available via
//! [`Container::signatures`] and files via [`Container::data_files`].
//!
//! Use the methods defined on [`Signature`] to inspect signatures and validate them. Detailed
//! validity information is exposed via [`Signature::validator_result`].
//!
//! Use the methods defined on [`DataFile`] to inspect data files. You can save a file to disk
//! with [`DataFile::save_as`] or provide a [`Write`](std::io::Write) implementation to
//! [`DataFile::write_to`].
//!
//! ```no_run
//! use digidocpp::Container;
//!
//! fn main() -> Result<(), digidocpp::Error> {
//!     digidocpp::initialize()?;
//!     let container = Container::open("path/to/document.asice")?;
//!
//!     for signature in container.signatures()? {
//!         println!("Signed by: {}", signature.signed_by()?);
//!         println!("Signed at: {}", signature.trusted_signing_time()?);
//!         println!("Validity: {:?}", signature.validator_result()?.status());
//!     }
//!
//!     for file in container.data_files()? {
//!         println!("File name: {}", file.file_name()?);
//!         println!("File size: {}", file.file_size()?);
//!         let mut contents = Vec::new();
//!         file.write_to(&mut contents)?;
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Creating and signing a container
//!
//! You can create a container with [`Container::create`] and add files with
//! [`Container::add_data_file`] or [`Container::add_data_file_from_reader`].
//!
//! Signing requires a signer instance. Standard signers are [`Pkcs11Signer`] and
//! `WinSigner` (Windows). These require a smart card to be accessible (see upstream manual's
//! [section on signing](https://open-eid.github.io/libdigidocpp/manual.html#API-add-sign)). Call
//! [`Container::sign`] with a signer for the signature to be added.
//!
//! ```no_run
//! use digidocpp::{Container, Pkcs11Signer};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     digidocpp::initialize()?;
//!
//!     let mut container = Container::create("contract.asice")?;
//!     container.add_data_file("path/to/contract.pdf", "application/pdf")?;
//!     let mut signer = Pkcs11Signer::new()?;
//!
//!     signer.set_pin(std::env::var("PIN2")?)?;
//!     container.sign(&mut signer)?;
//!     container.save()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Configuring the library
//!
//! Configuration is based on XML files described in the
//! [libdigidocpp manual](https://open-eid.github.io/libdigidocpp/manual.html#conf). You can load
//! the config from a file with [`Conf::from_file`]. Setters on [`Conf`] allow you to override
//! values from the XML. For the configuration to take effect, it must be passed to [`init_conf`]
//! after which the library must initialized or reinitialized via [`initialize`] (or other
//! initialize-function variant).
//!
//! ```no_run
//! use digidocpp::{Conf, LogLevel};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut conf = Conf::from_file("path/to/digidocpp.conf")?;
//!
//!     conf.set_tsl_cache("path/to/tsl")?;
//!     conf.set_log_file("path/to/digidocpp.log")?;
//!     conf.set_log_level(LogLevel::Debug);
//!     digidocpp::init_conf(conf)?;
//!     digidocpp::initialize()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Installation
//!
//! libdigidocpp is linked dynamically. It is not bundled, and there is no `vendored` or `static`
//! option.
//!
//! ## Requirements
//!
//! - libdigidocpp 4.5.0 or newer, with headers.
//! - A C++20 compiler.
//! - Rust 1.88 or newer.
//!
//! ## Finding libdigidocpp
//!
//! The build script finds libdigidocpp with [`Install::discover`]. A standard install needs no
//! configuration. These environment variables override the search. When set, they skip all
//! defaults:
//!
//! - `DIGIDOCPP_INCLUDE_DIR` and `DIGIDOCPP_LIB_DIR`. Set both.
//! - `DIGIDOCPP_FRAMEWORK_DIR`: directory that contains `digidocpp.framework` (Apple targets).
//! - `DIGIDOCPP_DIR`: install root (Windows targets, ignored elsewhere).
//!
//! Without them, the defaults are:
//!
//! - Linux: `pkg-config libdigidocpp` if available, otherwise `/usr/local` and `/usr`.
//! - macOS: `/Library/Frameworks/digidocpp.framework`, then `pkg-config libdigidocpp` if available,
//!   otherwise `/opt/homebrew` and `/usr/local`.
//! - Windows: `%ProgramFiles%\libdigidocpp` / `libdigidocpp-arm64` / `%ProgramFiles(x86)%`.
//!
//! ## Run time
//!
//! The loader must find libdigidocpp at start-up.
//!
//! - Linux and macOS: call [`Install::emit_rpath`] in the build script of your program or install
//!   the library in a system directory.
//! - Windows: put the DLL directory in `PATH`, or next to the executable.
//!
//! # Details
//!
//! ## Overview
//!
//! - [`Container`] is the primary entry point to any operations involving containers. Use
//!   [`Container::open`] or [`Container::create`] to get an instance.
//! - [`DataFile`] is a representation of a file in a container, owned by the container.
//! - [`Signature`] is a representation of a signature in a container, owned by the container.
//! - [`PartialSigner`] and [`Signer`] represent entities capable of holding signing parameters
//!   and creating signatures, respectively. Implementations are [`Pkcs11Signer`], `WinSigner`
//!   (Windows-only), [`ExternalPartialSigner`] and [`Pkcs12Signer`] (legally void). Use associated
//!   functions to get an instance.
//! - The validity of a signatures is not necessarily a boolean, see the upstream manual's
//!   [section on validation](https://open-eid.github.io/libdigidocpp/manual.html#validate) to read
//!   more. Validation using [`Signature::validate`] returns an error even if the signature is
//!   "VALID WITH WARNINGS". [`Signature::validator_result`] is a more ergonomic way of determining
//!   a signature's validity status.
//! - Signers sign a container with [`Container::sign`]: [`Pkcs12Signer`] (key file),
//!   [`Pkcs11Signer`] (ID-card) and `WinSigner` (Windows). [`ExternalPartialSigner`] takes the
//!   signature value from another service.
//! - [`Error`] wraps libdigidocpp failures, which form a tree structure.
//! - The upstream library must be initialized with [`initialize`] before it can be used.
//!   [`init_conf`] may be called with [`Conf`] beforehand to configure the library.
//! - Upstream libdigidocpp may connect to the network while performing tasks. It loads trust lists
//!   and validates certain containers online unless explicitly disallowed.
//!
//! ## Initialization and configuration
//!
//! The upstream library MUST be initialized before it can be used. This is requirement that flows
//! from their implementation. See the
//! [relevant section](https://open-eid.github.io/libdigidocpp/manual.html#initialization) in their
//! manual. We expose several `initialize*` functions for this purpose, the simplest of which is
//! [`initialize`]. This may be called multiple times, ensure that this is called at least once.
//!
//! Upstream has comprehensive configuration capabilities via XML configuration files. See the
//! [relevant section](https://open-eid.github.io/libdigidocpp/manual.html#conf) in their manual
//! and consult their
//! [API reference](https://open-eid.github.io/libdigidocpp/classdigidoc_1_1XmlConfV5.html). For
//! configuration to take effect, it must be installed by calling [`init_conf`], followed by a
//! call to [`initialize`]. Note that upstream searches for the configuration XML from
//! system/user directories by default unless you specify a path explicitly via [`Conf::from_file`].
//!
//! Upstream also has a termination function, exposed as [`terminate`], which may be optionally
//! called to direct upstream to close opened libraries and delete temporary files. Using the
//! library after this has been called is UNSAFE.
//!
//! ## Errors
//!
//! Any call into libdigidocpp can fail, because the C++ code throws. We handle this by relying on
//! [libdigidocpp-sys] to serialize exceptions into JSON on the C++ side before passing them to
//! Rust. On the Rust side we deserialize these into [`Error::Library`] subvariants.
//!
//! Exceptions intentionally thrown upstream are generally instances of [`digidoc::Exception`] which
//! are turned into [`LibraryError::DigiDoc`] while unknown thrown values become
//! [`LibraryError::Unknown`]. [`LibraryError::DigiDoc`] exposes information about the failure mode
//! via [`DigiDocError::has_code`] and the tree of causes via [`DigiDocError::iter_tree`].
//!
//! Other error types include [`Error::Internal`] for invalid values rejected by this crate,
//! [`Error::Ffi`] for unexpected issues at the FFI boundary, and [`Error::Io`] for errors returned
//! by user-provided instances of [`Write`](std::io::Write)/[`Read`](std::io::Read).
//!
//! ## Thread safety
//!
//! Upstream libdigidocpp has global state and is not thread-safe in practice. This means that this
//! crate can only be called used in a single thread.
//!
//! [libdigidocpp]: https://www.id.ee/en/article/digidoc-libraries-c-library-libdigidocpp/
//! [libdigidocpp manual]: https://open-eid.github.io/libdigidocpp/
//! [`Install::discover`]: https://docs.rs/libdigidocpp-build/latest/libdigidocpp_build/struct.Install.html#method.discover
//! [`Install::emit_rpath`]: https://docs.rs/libdigidocpp-build/latest/libdigidocpp_build/struct.Install.html#method.emit_rpath
//! [ASiC-E]: https://open-eid.github.io/libdigidocpp/manual.html#container
//! [ASiC-S]: https://open-eid.github.io/libdigidocpp/manual.html#asics
//! [BDOC]: https://open-eid.github.io/libdigidocpp/manual.html#profiles
//! [XAdES]: https://en.wikipedia.org/wiki/XAdES
//! [CAdES]: https://en.wikipedia.org/wiki/CAdES_(computing)
//! [PKCS #11]: https://open-eid.github.io/libdigidocpp/manual.html#pkcs11-settings
//! [PKCS #12]: https://en.wikipedia.org/wiki/PKCS_12
//! [libdigidocpp-sys]: https://crates.io/crates/libdigidocpp-sys
//! [`digidoc::Exception`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Exception.html
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

mod algorithm;
mod conf;
mod container;
mod data_file;
mod der_cert;
mod exception;
mod init;
mod signature;
mod signer;
mod stream;
mod util;

pub use algorithm::*;
pub use conf::*;
pub use container::*;
pub use data_file::*;
pub use der_cert::*;
pub use exception::*;
pub use init::*;
pub use signature::*;
pub use signer::*;
