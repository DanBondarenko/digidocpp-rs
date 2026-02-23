use cxx::let_cxx_string;
use libdigidocpp_sys::container::ffi;
use libdigidocpp_sys::exception::FfiException;
use crate::DataFile;

pub struct Container(cxx::UniquePtr<ffi::Container>);

impl Container {
    pub fn open(path: &str) -> Result<Self, FfiException> {
        let_cxx_string!(path = path);
        crate::fallible(|| ffi::Container::openPtr(&*path)).map(Self)
    }

    pub fn create(path: &str) -> Result<Self, FfiException> {
        let_cxx_string!(path = path);
        crate::fallible(|| ffi::Container::createPtr(&*path)).map(Self)
    }

    pub fn media_type(&self) -> Result<String, FfiException> {
        crate::fallible(|| ffi::media_type(&*self.0))
    }

    pub fn data_files(&self) -> Result<Vec<DataFile<'_>>, FfiException> {
        let count = crate::fallible(|| ffi::data_files_size(&self.0))?;
        let data_files = (0..count)
            .map(|index| ffi::data_file_at(&self.0, index).map(DataFile) )
            .collect::<Result<_, _>>()?;

        Ok(data_files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;
    use serial_test::serial;

    #[test]
    #[serial]
    fn opens_a_real_container() -> Result<(), Box<dyn Error>> {
        crate::initialize_default()?;

        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/test.asice");
        let container = Container::open(path)?;

        assert_eq!(dbg!(container.media_type()?), "application/vnd.etsi.asic-e+zip");

        let data_files = container.data_files()?;
        assert_eq!(data_files.len(), 1);

        let data_file = &data_files[0];
        assert_eq!(dbg!(data_file.id()?), "test1.txt");
        assert_eq!(dbg!(data_file.file_name()?), "test1.txt");
        assert_eq!(dbg!(data_file.file_size()?), 5);

        Ok(())
    }
}