#![allow(
    dead_code,
    reason = "each integration test binary uses a subset of these helpers"
)]

use bon::Builder;
use itertools::Itertools;
use libdigidocpp::{Container, DataFile};
use std::error::Error;
use std::ffi::OsString;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

#[allow(unused_macros)]
macro_rules! assert_general_digidoc_err {
    ($val:expr $(,)?) => {
        match $val {
            result => {
                let Err(::libdigidocpp::Error::Library(::libdigidocpp::LibraryError::DigiDoc(error))) = result else {
                    ::core::panic!("Expected digidoc error, instead got {result:?}");
                };
                ::core::assert_eq!(error.code(), Some(::libdigidocpp::ExceptionCode::General));
                error
            }
        }
    };
    ($val:expr, $($arg:tt)+) => {
        match $val {
            result => {
                let Err(::libdigidocpp::Error::Library(::libdigidocpp::LibraryError::DigiDoc(error))) = result else {
                    ::core::panic!(
                        "{}: Expected digidoc error, instead got {result:?}",
                        ::core::format_args!($($arg)+)
                    );
                };
                ::core::assert_eq!(error.code(), Some(::libdigidocpp::ExceptionCode::General));
                error
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use assert_general_digidoc_err;

pub(crate) fn fixture_path(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);
    assert!(
        path.exists(),
        "Path {} doesn't exist",
        path.to_str().expect("Can't produce string from path")
    );

    path.into_os_string()
        .into_string()
        .expect("Can't produce string from path")
}

pub(crate) fn assert_if_some<T: PartialEq + Debug + ?Sized>(left: &T, right: Option<&T>) {
    if let Some(value) = right {
        assert_eq!(left, value);
    }
}

#[derive(Builder)]
#[builder(on(_, into))]
pub(crate) struct ContainerExpectation {
    pub(crate) media_type: Option<String>,
    pub(crate) data_files: Option<Vec<DataFileExpectation>>,
}

impl ContainerExpectation {
    pub(crate) const MEDIA_TYPE_ASICE: &str = "application/vnd.etsi.asic-e+zip";
    pub(crate) const MEDIA_TYPE_ASICS: &str = "application/vnd.etsi.asic-s+zip";

    pub(crate) fn assert(&self, container: &Container) -> Result<(), Box<dyn Error>> {
        if let Some(expected_media_type) = &self.media_type {
            assert_eq!(&container.media_type()?, expected_media_type);
        }

        if let Some(expected_data_files) = &self.data_files {
            let data_files = container.data_files()?;
            assert_eq!(data_files.len(), expected_data_files.len());
            expected_data_files
                .iter()
                .zip_eq(data_files)
                .try_for_each(|(expectation, data_file)| expectation.assert(&data_file))?;
        }

        Ok(())
    }
}

#[derive(Builder)]
#[builder(on(_, into))]
pub(crate) struct DataFileExpectation {
    pub(crate) id: String,
    pub(crate) file_name: String,
    pub(crate) file_size: u64,
    pub(crate) media_type: String,
    pub(crate) calc_digest_default: Option<Vec<u8>>,
}

impl DataFileExpectation {
    pub(crate) const MEDIA_TYPE_OCTET_STREAM: &str = "application/octet-stream";
    pub(crate) const MEDIA_TYPE_TEXT_PLAIN: &str = "text/plain";

    pub(crate) fn assert(&self, data_file: &DataFile) -> Result<(), Box<dyn Error>> {
        assert_eq!(data_file.id()?, self.id);
        assert_eq!(data_file.file_name()?, self.file_name);
        assert_eq!(data_file.file_size()?, self.file_size);
        assert_eq!(data_file.media_type()?, self.media_type);
        assert_if_some(
            &data_file.calc_digest_default()?,
            self.calc_digest_default.as_ref(),
        );
        Ok(())
    }
}

#[cfg(unix)]
pub(crate) fn non_utf8_path() -> PathBuf {
    use std::os::unix::ffi::OsStringExt;

    PathBuf::from(OsString::from_vec(b"invalid-\xff.asice".to_vec()))
}

#[cfg(windows)]
pub(crate) fn non_utf8_path() -> PathBuf {
    use std::os::windows::ffi::OsStringExt;

    // 0xD800 is a lone surrogate, so it is not valid UTF-16.
    PathBuf::from(OsString::from_wide(&[0x0069, 0xD800, 0x002E, 0x0061]))
}
