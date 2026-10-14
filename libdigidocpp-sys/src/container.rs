//! Containers: upstream [`digidoc::Container`], bound as [`Container`], and its factory
//! functions.
//!
//! [`digidoc::Container`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Container.html

pub use crate::container_callback::open_with_callback_resuming_panic;
use ffi::Container;
use std::fmt::{self, Debug, Formatter};

/// Bridge for containers.
#[cxx::bridge]
#[allow(clippy::missing_errors_doc, reason = "cxx")]
#[allow(clippy::exhaustive_structs, reason = "cxx")]
pub mod ffi {
    /// Outcome of upstream `Container::extendContainerValidity`.
    #[namespace = "digidoc_rs::container"]
    #[derive(Debug)]
    struct ExtendedValidity {
        /// The new ASiC-S container, or null if the original was extended in place.
        container: UniquePtr<Container>,
        /// Number of signatures extended in place, or 0 if the container was wrapped.
        extended_count: usize,
    }

    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("digidocpp/Container.h");

        /// Upstream [`digidoc::Container`].
        ///
        /// [`digidoc::Container`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Container.html
        type Container;

        /// Upstream [`digidoc::DataFile`].
        ///
        /// [`digidoc::DataFile`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1DataFile.html
        type DataFile = crate::data_file::ffi::DataFile;

        /// Upstream [`digidoc::Signature`].
        ///
        /// [`digidoc::Signature`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signature.html
        type Signature = crate::signature::ffi::Signature;

        /// Upstream [`digidoc::Signer`].
        ///
        /// [`digidoc::Signer`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signer.html
        type Signer = crate::signer::ffi::Signer;
    }

    #[namespace = "digidoc_rs::stream"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/src/stream.rs.h");

        /// A seekable reader, see [`Source`](crate::stream::Source).
        type Source = crate::stream::Source;
    }

    #[namespace = "digidoc_rs::container"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/exception_ffi.h");
        include!("libdigidocpp-sys/include/container_ffi.h");

        /// Calls upstream `Container::openPtr`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when the file is
        /// missing or not a supported container.
        fn open(path: &str) -> Result<UniquePtr<Container>>;

        /// Calls upstream `Container::createPtr`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn create(path: &str) -> Result<UniquePtr<Container>>;

        /// Calls upstream `Container::mediaType`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn media_type(container: &Container) -> Result<String>;

        /// Returns the length of upstream `Container::dataFiles`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn data_files_size(container: &Container) -> Result<usize>;

        /// Returns element `index` of upstream `Container::dataFiles`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when `index` is out of
        /// range.
        fn data_file_at(container: &Container, index: usize) -> Result<&DataFile>;

        /// Returns the length of upstream `Container::signatures`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn signatures_size(container: &Container) -> Result<usize>;

        /// Returns element `index` of upstream `Container::signatures`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when `index` is out of
        /// range.
        fn signature_at(container: &Container, index: usize) -> Result<&Signature>;

        /// Returns a mutable reference to element `index` of upstream `Container::signatures`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when `index` is out of
        /// range.
        fn signature_at_mut(
            container: Pin<&mut Container>,
            index: usize,
        ) -> Result<Pin<&mut Signature>>;

        /// Calls upstream `Container::removeSignature`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when `index` is out of
        /// range.
        fn remove_signature(container: Pin<&mut Container>, index: u32) -> Result<()>;

        /// Calls upstream `Container::extendContainerValidity`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when the container has
        /// no extendable signatures.
        fn extend_container_validity(
            container: Pin<&mut Container>,
            signer: Pin<&mut Signer>,
        ) -> Result<ExtendedValidity>;

        /// Calls upstream `Container::sign`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws or returns null, for example when
        /// the container has no data files or signing fails.
        fn sign<'a>(
            container: Pin<&'a mut Container>,
            signer: Pin<&mut Signer>,
        ) -> Result<Pin<&'a mut Signature>>;

        /// Calls upstream `Container::prepareSignature`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws or returns null, for example when
        /// the container has no data files or is not ASiC-E.
        fn prepare_signature<'a>(
            container: Pin<&'a mut Container>,
            signer: Pin<&mut Signer>,
        ) -> Result<Pin<&'a mut Signature>>;

        /// Calls upstream `Container::addDataFile`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when the container has
        /// signatures, the file is missing or empty, or a data file with that name exists.
        fn add_data_file(
            container: Pin<&mut Container>,
            path: &str,
            media_type: &str,
        ) -> Result<()>;

        /// Calls upstream `Container::removeDataFile`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when the container has
        /// signatures or `index` is out of range.
        fn remove_data_file(container: Pin<&mut Container>, index: u32) -> Result<()>;

        /// Calls upstream `Container::addAdESSignature`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when the container has
        /// no data files or is not ASiC-E.
        fn add_ades_signature(
            container: Pin<&mut Container>,
            xades_signatures: &[u8],
        ) -> Result<()>;

        /// Calls upstream `Container::save`.
        ///
        /// An empty `path` keeps the current one.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when the container has
        /// no data files or the file cannot be created.
        fn save(container: Pin<&mut Container>, path: &str) -> Result<()>;

        /// Calls upstream `Container::addDataFile`, with `reader` wrapped as an `istream`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when the container has
        /// signatures, `name` contains a path separator, or a data file with that name exists.
        fn add_data_file_from_reader(
            container: Pin<&mut Container>,
            name: &str,
            media_type: &str,
            reader: Box<Source>,
        ) -> Result<()>;
    }
}

impl Debug for Container {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Container").finish_non_exhaustive()
    }
}
