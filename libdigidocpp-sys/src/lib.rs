//! Low-level Rust bindings to [libdigidocpp], the C++ library for creating, signing and validating
//! digitally signed containers. Built with [`cxx`].
//!
//! # Example
//!
//! ```no_run
//! use libdigidocpp_sys::{container, init, signature};
//!
//! fn main() -> Result<(), cxx::Exception> {
//!     init::ffi::initialize_default()?;
//!
//!     let container = container::ffi::open("path/to/document.asice")?;
//!     let first_signature = container::ffi::signature_at(&container, 0)?;
//!     let is_valid = signature::ffi::validate(first_signature).is_ok();
//!
//!     println!("{is_valid}");
//!     Ok(())
//! }
//! ```
//!
//! # Where to find things
//!
//! - Configure and initialize the library: [`conf::ffi::SettableConf`], [`conf::ffi::init_conf`],
//!   then [`init::ffi::initialize_default`].
//! - Open, create or save a container: [`container::ffi::open`], [`container::ffi::create`], [`container::ffi::save`].
//! - Read data files: [`container::ffi::data_file_at`], operations in [`data_file::ffi`].
//! - Validate signatures: [`container::ffi::signature_at`], then [`signature::ffi::validate`]
//!   or [`signature::ffi::validator`].
//! - Sign: make a signer with [`signer::ffi::make_pkcs11_signer`] or
//!   [`signer::ffi::make_win_signer`], then [`container::ffi::sign`].
//! - Stream data through Rust readers and writers: [`stream`].
//!
//! # From the C++ API
//!
//! - Classes keep their names. C++ methods are represented as free functions that take the object first.
//! - A list becomes a `*_size` function and an `*_at` function.
//! - Paths and strings are `&str`, so they must be UTF-8.
//!
//! # Errors
//! A call that can throw returns `Result<_, cxx::Exception>`. The message is JSON. Decode it with
//! [`exception::ffi::ExceptionData::from`] to get the code, file, line and causes.
//!
//! # Usage rules
//!
//! - Optionally apply configuration via [`conf::ffi::init_conf`]. Upstream defaults to looking for
//!   configuration in system and user directories.
//! - Call an `initialize*` function of [`init::ffi`] after configuration has been applied,
//!   but before calling anything else.
//! - Optionally call [`init::ffi::terminate`] once at the end. No call is valid afterwards.
//! - Use the library from one thread. Upstream keeps global state and has no synchronization.
//!
//! # See also
//!
//! - The upstream C++ [API reference](https://open-eid.github.io/libdigidocpp/).
//!
//! [libdigidocpp]: https://github.com/open-eid/libdigidocpp
#![expect(clippy::multiple_unsafe_ops_per_block, reason = "cxx")]

pub mod conf;
pub mod container;
mod container_callback;
pub mod data_file;
pub mod exception;
pub mod init;
pub mod signature;
pub mod signer;
pub mod stream;
