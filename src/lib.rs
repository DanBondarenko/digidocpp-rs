//! Rust bindings for [libdigidocpp], the C++ library behind the Estonian DigiDoc software.
//!
//! Create, sign, validate and extend ASiC-E containers (XAdES signatures, RFC 3161
//! time-stamps). [`ContainerType`] lists the other formats that the library reads and validates.
//! The C++ API is documented in the [upstream reference].
//!
//! Unofficial. Not affiliated with the Estonian Information System Authority (RIA).
//!
//! # Quick start
//!
//! ```no_run
//! use digidocpp::Container;
//!
//! fn main() -> Result<(), digidocpp::Error> {
//!     digidocpp::initialize()?;
//!     let container = Container::open("document.asice")?;
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
//! # Signing
//!
//! Create a container, add a data file and sign it with a PKCS #12 file. The example reads the
//! password from an environment variable.
//!
//! ```no_run
//! use digidocpp::{Container, Pkcs12Signer};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     digidocpp::initialize()?;
//!     let mut container = Container::create("signed.asice")?;
//!     container.add_data_file("document.pdf", "application/pdf")?;
//!
//!     let mut signer = Pkcs12Signer::new("signer.p12", std::env::var("P12_PASSWORD")?)?;
//!     container.sign(&mut signer)?;
//!     container.save()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Overview
//!
//! - [`Container`] holds data files and signatures. Use [`Container::open`] or
//!   [`Container::create`].
//! - [`Signature`] gives signer, time and validity. [`Signature::validator_result`] tells warnings
//!   from errors.
//! - Signers sign a container with [`Container::sign`]: [`Pkcs12Signer`] (key file),
//!   [`Pkcs11Signer`] (ID-card) and `WinSigner` (Windows). [`ExternalPartialSigner`] takes the
//!   signature value from another service.
//! - [`Error`] wraps libdigidocpp failures. Use [`Error::has_code`] to test for one, such as a
//!   wrong PIN. Use [`Error::as_digidoc`] to read the cause tree.
//!
//! # Errors
//! Any call into libdigidocpp can fail, because the C++ code throws. The `# Errors` section of a
//! function lists notable causes only. [`Error::Internal`] and [`Error::Io`] mark failures that
//! this crate detects itself.
//!
//! # Notes
//!
//! - For customizing configuration, call [`init_conf`] with a [`Conf`] before anything else.
//! - Call [`initialize`] before using the [`Container`] API. It loads the trust list (TSL) and can
//!   use the network.
//! - Optionally, call [`terminate`] at the end. It may only be called once (hence it's `unsafe`).
//!   Any calls into the library afterwards may crash the process.
//! - Upstream libdigidocpp is not thread-safe in practice. Use it from one thread.
//! - Paths must be valid UTF-8.
//!
//! # Building
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
//! - Linux and macOS: call [`Install::emit_rpath`] in the build script of your program, or install
//!   the library in a system directory. Skip the call when the program runs on another machine.
//! - Windows: put the DLL directory in `PATH`, or next to the executable.
//!
//! [libdigidocpp]: https://github.com/open-eid/libdigidocpp
//! [upstream reference]: https://open-eid.github.io/libdigidocpp/
//! [`Install::discover`]: https://docs.rs/libdigidocpp-build/latest/libdigidocpp_build/struct.Install.html#method.discover
//! [`Install::emit_rpath`]: https://docs.rs/libdigidocpp-build/latest/libdigidocpp_build/struct.Install.html#method.emit_rpath
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]

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
