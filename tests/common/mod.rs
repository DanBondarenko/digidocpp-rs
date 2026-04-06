#![allow(
    dead_code,
    reason = "each integration test binary uses a subset of these helpers"
)]

use bon::Builder;
use itertools::Itertools;
use libdigidocpp::{Container, DataFile};
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;

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
    pub(crate) media_type: String,
    pub(crate) data_files: Vec<DataFileExpectation>,
}

impl ContainerExpectation {
    pub(crate) const MEDIA_TYPE_ASICE: &str = "application/vnd.etsi.asic-e+zip";
    pub(crate) const MEDIA_TYPE_ASICS: &str = "application/vnd.etsi.asic-s+zip";

    pub(crate) fn assert(&self, container: &Container) -> Result<(), Box<dyn Error>> {
        assert_eq!(container.media_type()?, self.media_type);

        let data_files = container.data_files()?;
        assert_eq!(data_files.len(), self.data_files.len());
        self.data_files
            .iter()
            .zip_eq(data_files)
            .try_for_each(|(expectation, data_file)| expectation.assert(&data_file))?;

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

    pub(crate) fn test1_expectation() -> Self {
        Self::builder()
            .id("test1.txt")
            .file_name("test1.txt")
            .file_size(5u64)
            .media_type(Self::MEDIA_TYPE_OCTET_STREAM)
            .build()
    }
}
