//! Safe bindings to [libdigidocpp]: create and validate eIDAS digital signatures in ASiC-E/XAdES
//! containers.
//!
//! All major workflows of libdigidocpp are exposed to Rust. This includes:
//! - Opening existing containers and validating the digital signatures or timestamps contained
//!   therein.
//! - Creating containers from files and adding digital signatures using either [PKCS #11],
//!   [PKCS #12], Windows `CryptoAPI`, or an external signer (e.g. mobile ID).
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
//! use digidocpp::{Container, Library, Uninitialized};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let library = Library::<Uninitialized>::acquire()?.initialize()?;
//!     let container = Container::open(&library, "path/to/document.asice")?;
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
//! use digidocpp::signer::Pkcs11Signer;
//! use digidocpp::{Container, Library, Uninitialized};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let library = Library::<Uninitialized>::acquire()?.initialize()?;
//!
//!     let mut container = Container::create(&library, "contract.asice")?;
//!     container.add_data_file("path/to/contract.pdf", "application/pdf")?;
//!     let mut signer = Pkcs11Signer::new(&library)?;
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
//! the config from a file with [`Conf::from_file`]. Setters on [`Conf`] allow you to
//! override values from the XML. For configuration to take effect, pass it to
//! [`Library<Uninitialized>::configure`], then initialize with [`Library<Uninitialized>::initialize`] (or other `initialize*`
//! variant). Without configuration upstream installs its default.
//!
//! ```no_run
//! use digidocpp::conf::{Conf, LogLevel};
//! use digidocpp::{Library, Uninitialized};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut library = Library::<Uninitialized>::acquire()?;
//!     let mut conf = Conf::from_file(&library, "path/to/digidocpp.conf")?;
//!
//!     conf.set_tsl_cache("path/to/tsl")?;
//!     conf.set_log_file("path/to/digidocpp.log")?;
//!     conf.set_log_level(LogLevel::Debug);
//!     library.configure(conf)?;
//!     let library = library.initialize()?;
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
//! - Every call into upstream needs a [`Library`] handle which only one thread may have at a time.
//!   The handle can be obtained with [`acquire`]/[`try_acquire`] or associated functions on
//!   [`Library<Uninitialized>`] and [`Library<Initialized>`].
//! - Upstream libdigidocpp may connect to the network while performing tasks. It loads trust lists
//!   and validates certain containers online unless explicitly disallowed.
//!
//! ## Initialization and configuration
//!
//! This library has process-level state: it's either uninitialized, initialized or terminated.
//! These state transitions are irreversible and persist for the duration of the process.
//!
//! The library must be initialized before use. This is enforced with typestate on
//! [`Library<Uninitialized>`] and [`Library<Initialized>`]. Call
//! [`Library<Uninitialized>::initialize`] or sibling methods to initialize. The library can only be
//! configured before it's initialized. Call [`Library<Uninitialized>::configure`] to apply
//! configuration.
//!
//! Upstream has comprehensive configuration capabilities via XML configuration files. See the
//! [relevant section](https://open-eid.github.io/libdigidocpp/manual.html#conf) in their manual
//! and consult their
//! [API reference](https://open-eid.github.io/libdigidocpp/classdigidoc_1_1XmlConfV5.html). Note
//! that upstream searches for configuration XML from system/user directories by default unless you
//! specify path explicitly via [`Conf::from_file`].
//!
//! Initialization is required by upstream libdigidocpp, see
//! [the libdigidocpp manual](https://open-eid.github.io/libdigidocpp/manual.html#initialization)
//! for more information on these constraints. Upstream also has termination function, exposed as
//! [`Library<Initialized>::terminate`]. It may be optionally called to direct upstream to close
//! opened libraries and delete temporary files. Termination is permanent: acquiring the library
//! handle fails with [`AccessError::Terminated`] on every thread afterwards.
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
//! Upstream libdigidocpp has global mutable state without thread safety. This crate puts a
//! process-wide lock in front of all upstream calls to ensure thread safety. Calls into upstream
//! API require the [`Library`] handle which only one thread may have at a time. It is `!Send`
//! and `!Sync`.
//!
//! [`acquire`]/[`Library<Uninitialized>::acquire`]/[`Library<Initialized>::acquire`] block until
//! the library handle becomes available, while
//! [`try_acquire`]/[`Library<Uninitialized>::try_acquire`]/[`Library<Initialized>::try_acquire`]
//! fail with [`AccessError::WouldBlock`] instead of waiting. A thread that already holds
//! the library calling these methods gets [`AccessError::AlreadyHeld`] to prevent deadlock.
//!
//! The library's state persists for the duration of the process. Dropping handle only releases lock.
//! Once library has been initialized, it remains so for the duration of the process or until
//! terminated.
//!
//! Hold handle for short time only as other threads may library wait for it.
//! Since [`acquire`]/[`Library<Uninitialized>::acquire`]/[`Library<Initialized>::acquire`] blocks
//! the thread and handles can't be held across `.await`, it is recommended to call the library
//! inside a blocking task instead. Acquire handle in closure, do all library work, drop handle,
//! return owned data.
//!
//! ```no_run
//! # use digidocpp::{Initialized, Library};
//! # async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let signers = tokio::task::spawn_blocking(|| -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
//!     let library = Library::<Initialized>::acquire()?;
//!     let container = digidocpp::Container::open(&library, "contract.asice")?;
//!     let signers = container
//!         .signatures()?
//!         .iter()
//!         .map(|signature| signature.signed_by())
//!         .collect::<Result<_, _>>()?;
//!     Ok(signers)
//! });
//! # let _ = signers.await??;
//! # Ok(())
//! # }
//! ```
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
//! [`Pkcs11Signer`]: signer::Pkcs11Signer
//! [`Pkcs12Signer`]: signer::Pkcs12Signer
//! [`ExternalPartialSigner`]: signer::ExternalPartialSigner
//! [`Conf`]: conf::Conf
//! [`Conf::from_file`]: conf::Conf::from_file
//! [`LibraryError::DigiDoc`]: error::LibraryError::DigiDoc
//! [`LibraryError::Unknown`]: error::LibraryError::Unknown
//! [`DigiDocError::has_code`]: error::DigiDocError::has_code
//! [`DigiDocError::iter_tree`]: error::DigiDocError::iter_tree
//! [`acquire`]: library::acquire
//! [`try_acquire`]: library::try_acquire
//! [`Access`]: library::Access
//! [`AccessError::AlreadyHeld`]: library::AccessError::AlreadyHeld
//! [`AccessError::AlreadyInitialized`]: library::AccessError::AlreadyInitialized
//! [`AccessError::Terminated`]: library::AccessError::Terminated
//! [`AccessError::WouldBlock`]: library::AccessError::WouldBlock
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

mod data_file;
mod der_cert;
mod stream;
mod util;

pub mod algorithm;
pub mod conf;
pub mod container;
pub mod error;
pub mod library;
pub mod signature;
pub mod signer;

#[doc(inline)]
pub use container::Container;
#[doc(inline)]
pub use data_file::DataFile;
#[doc(inline)]
pub use der_cert::DerCert;
#[doc(inline)]
pub use error::Error;
#[doc(inline)]
pub use library::{Initialized, Library, Uninitialized};
#[doc(inline)]
pub use signature::Signature;
#[doc(inline)]
pub use signer::{PartialSigner, Signer};
