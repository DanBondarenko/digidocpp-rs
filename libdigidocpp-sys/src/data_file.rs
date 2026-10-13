//! Data files inside a container: upstream [`digidoc::DataFile`], bound as [`ffi::DataFile`].
//!
//! [`digidoc::DataFile`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1DataFile.html

use std::fmt::{self, Debug, Formatter};

/// Bridge for data files.
#[cxx::bridge]
#[allow(clippy::missing_errors_doc, reason = "cxx")]
pub mod ffi {
    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("digidocpp/DataFile.h");

        /// Upstream [`digidoc::DataFile`], owned by its container.
        ///
        /// [`digidoc::DataFile`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1DataFile.html
        type DataFile;
    }

    #[namespace = "digidoc_rs::stream"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/src/stream.rs.h");

        /// A writer, see [`Sink`](crate::stream::Sink).
        type Sink<'a> = crate::stream::Sink<'a>;
    }

    #[namespace = "digidoc_rs::data_file"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/data_file_ffi.h");
        include!("libdigidocpp-sys/include/exception_ffi.h");

        /// Calls upstream `DataFile::id`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn id(data_file: &DataFile) -> Result<String>;

        /// Calls upstream `DataFile::mediaType`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn media_type(data_file: &DataFile) -> Result<String>;

        /// Calls upstream `DataFile::fileName`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn file_name(data_file: &DataFile) -> Result<String>;

        /// Calls upstream `DataFile::fileSize`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn file_size(data_file: &DataFile) -> Result<u64>;

        /// Calls upstream `DataFile::calcDigest`.
        ///
        /// An empty `method` selects the configured default.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn calc_digest(data_file: &DataFile, method: &str) -> Result<Vec<u8>>;

        /// Calls upstream `DataFile::saveAs`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn save_as(data_file: &DataFile, path: &str) -> Result<()>;

        /// Calls upstream `DataFile::saveAs`, with `sink` as the `ostream`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn write_to(data_file: &DataFile, sink: &mut Sink<'_>) -> Result<()>;
    }
}

impl Debug for ffi::DataFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataFile").finish_non_exhaustive()
    }
}
