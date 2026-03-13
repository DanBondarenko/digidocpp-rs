use crate::DataFile;
use cxx::let_cxx_string;
use libdigidocpp_sys::container::ffi;
use libdigidocpp_sys::exception::FfiException;

pub struct Container(cxx::UniquePtr<ffi::Container>);

impl Container {
    pub fn open(path: &str) -> Result<Self, FfiException> {
        let_cxx_string!(path = path);
        Ok(ffi::Container::openPtr(&*path)?).map(Self)
    }

    pub fn create(path: &str) -> Result<Self, FfiException> {
        let_cxx_string!(path = path);
        Ok(ffi::Container::createPtr(&*path)?).map(Self)
    }

    pub fn media_type(&self) -> Result<String, FfiException> {
        Ok(ffi::media_type(&*self.0)?)
    }

    pub fn data_files(&self) -> Result<Vec<DataFile<'_>>, FfiException> {
        let count = ffi::data_files_size(&self.0)?;
        let data_files = (0..count)
            .map(|index| ffi::data_file_at(&self.0, index).map(DataFile))
            .collect::<Result<_, _>>()?;

        Ok(data_files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ExceptionCode;
    use serial_test::serial;
    use std::error::Error;

    struct SingleFileContainerExpectation<'a> {
        media_type: &'a str,
        file_id: &'a str,
        file_name: &'a str,
        file_size: u64,
    }

    type SFCE<'a> = SingleFileContainerExpectation<'a>;

    fn assert_opening_container_and_reading_contents(
        path: &str,
        expectations: SingleFileContainerExpectation,
    ) -> Result<(), Box<dyn Error>> {
        let container = Container::open(path)?;

        assert_eq!(dbg!(container.media_type()?), expectations.media_type);

        let data_files = container.data_files()?;
        assert_eq!(data_files.len(), 1);

        let data_file = &data_files[0];
        assert_eq!(dbg!(data_file.id()?), expectations.file_id);
        assert_eq!(dbg!(data_file.file_name()?), expectations.file_name);
        assert_eq!(dbg!(data_file.file_size()?), expectations.file_size);

        Ok(())
    }

    fn assert_opening_container_fails(
        path: &str,
    ) -> Result<(), Box<dyn Error>> {
        let container = Container::open(path);

        let Err(ffi_exception) = container else {
            panic!("opening invalid container should fail");
        };

        assert_eq!(dbg!(ffi_exception.code), ExceptionCode::General);
        dbg!(ffi_exception.message);

        Ok(())
    }

    #[test]
    #[serial]
    fn opens_asice_container_with_expired_certificates() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_and_reading_contents(
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/test.asice"),
            SFCE {
                media_type: "application/vnd.etsi.asic-e+zip",
                file_id: "test1.txt",
                file_name: "test1.txt",
                file_size: 5,
            },
        )
    }

    #[test]
    #[serial]
    fn opens_asics_container() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_and_reading_contents(
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/test.asics"),
            SFCE {
                media_type: "application/vnd.etsi.asic-s+zip",
                file_id: "test1.txt",
                file_name: "test1.txt",
                file_size: 5,
            },
        )
    }

    #[test]
    #[serial]
    fn opens_asics_container_with_metadata() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_and_reading_contents(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/test-meta.asics"
            ),
            SFCE {
                media_type: "application/vnd.etsi.asic-s+zip",
                file_id: "test1.txt",
                file_name: "test1.txt",
                file_size: 5,
            },
        )
    }

    #[test]
    #[serial]
    fn opens_asics_container_with_invalid_timestamp_token() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_and_reading_contents(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/test-invalidts.asics"
            ),
            SFCE {
                media_type: "application/vnd.etsi.asic-s+zip",
                file_id: "test2.txt",
                file_name: "test2.txt",
                file_size: 10,
            },
        )
    }

    #[test]
    #[serial]
    fn opens_asics_container_with_zero_size_contents() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_and_reading_contents(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/test-tera-empty.asics"
            ),
            SFCE {
                media_type: "application/vnd.etsi.asic-s+zip",
                file_id: "emptyFile.ddoc",
                file_name: "emptyFile.ddoc",
                file_size: 0,
            },
        )
    }

    #[test]
    #[serial]
    fn opens_asics_container_with_large_contents() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_and_reading_contents(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/test-tera.asics"
            ),
            SFCE {
                media_type: "application/vnd.etsi.asic-s+zip",
                file_id: "ddoc_for_testing.ddoc",
                file_name: "ddoc_for_testing.ddoc",
                file_size: 8736,
            },
        )
    }

    #[test]
    #[serial]
    fn opens_asice_container_with_subfolder_contents() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_and_reading_contents(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/asice-path.asice"
            ),
            SFCE {
                media_type: "application/vnd.etsi.asic-e+zip",
                file_id: "folder/test1.txt",
                file_name: "folder/test1.txt",
                file_size: 4,
            },
        )
    }

    #[test]
    #[serial]
    fn opens_asice_container_with_forged_signature() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_and_reading_contents(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/forged_lt.asice"
            ),
            SFCE {
                media_type: "application/vnd.etsi.asic-e+zip",
                file_id: "example-for-signing.txt",
                file_name: "example-for-signing.txt",
                file_size: 57,
            },
        )
    }

    #[test]
    #[serial]
    fn opens_asice_container() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_and_reading_contents(
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/tsl.asice"),
            SFCE {
                media_type: "application/vnd.etsi.asic-e+zip",
                file_id: "test.txt",
                file_name: "test.txt",
                file_size: 4,
            },
        )
    }

    #[test]
    #[serial]
    fn fails_opening_asics_container_with_invalid_mimetype() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_fails(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/test-invalid.asics"))
    }

    #[test]
    #[serial]
    fn fails_opening_asics_container_with_subfolder() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_fails(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/test-subfolder.asics"))
    }

    #[test]
    #[serial]
    fn fails_opening_asice_container_with_relative_path_in_contents() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_fails(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/asice-relative.asice"))
    }

    #[test]
    #[serial]
    fn fails_opening_asice_with_content_file_with_dot_name() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_fails(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/dot.asice"))
    }

    #[serial]
    fn fails_opening_asice_with_content_file_with_empty_name() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;
        assert_opening_container_fails(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/pt_empty.asice"))
    }
}
