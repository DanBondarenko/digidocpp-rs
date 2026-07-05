use crate::signature::Signature;
use crate::util::path_to_str;
use crate::{DataFile, Error};
use cxx::let_cxx_string;
use libdigidocpp_sys::container::ffi;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;

pub struct Container(cxx::UniquePtr<ffi::Container>);

impl Container {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path_to_str(path.as_ref())?;
        let_cxx_string!(path = path);
        Ok(Self(ffi::Container::openPtr(&path)?))
    }

    pub fn create(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path_to_str(path.as_ref())?;
        let_cxx_string!(path = path);
        Ok(Self(ffi::Container::createPtr(&path)?))
    }

    pub fn media_type(&self) -> Result<String, Error> {
        Ok(ffi::media_type(&self.0)?)
    }

    pub fn data_files(&self) -> Result<Vec<DataFile<'_>>, Error> {
        let count = ffi::data_files_size(&self.0)?;
        let data_files = (0..count)
            .map(|index| ffi::data_file_at(&self.0, index).map(DataFile))
            .collect::<Result<_, _>>()?;

        Ok(data_files)
    }

    pub fn add_data_file(&mut self, path: impl AsRef<Path>, media_type: &str) -> Result<(), Error> {
        let path = path_to_str(path.as_ref())?;
        ffi::add_data_file(self.0.pin_mut(), path, media_type)?;
        Ok(())
    }

    pub fn remove_data_file(&mut self, index: usize) -> Result<(), Error> {
        let index = u32::try_from(index).map_err(|_| {
            Error::Internal(format!("data file index does not fit in 32 bits: {index}"))
        })?;
        ffi::remove_data_file(self.0.pin_mut(), index)?;
        Ok(())
    }

    pub fn signatures(&self) -> Result<Vec<Signature<'_>>, Error> {
        let count = ffi::signatures_size(&self.0)?;
        let data_files = (0..count)
            .map(|index| ffi::signature_at(&self.0, index).map(Signature))
            .collect::<Result<_, _>>()?;

        Ok(data_files)
    }
}

impl Display for Container {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let media_type = self
            .media_type()
            .unwrap_or_else(|_| String::from("<unknown>"));
        let data_files_count = self.data_files().map_or_else(
            |_| String::from("<unknown>"),
            |files| files.len().to_string(),
        );
        let signatures_count = self.signatures().map_or_else(
            |_| String::from("<unknown>"),
            |signatures| signatures.len().to_string(),
        );
        write!(
            f,
            "Container of type {media_type} with {data_files_count} files and {signatures_count} signatures"
        )
    }
}

impl Debug for Container {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Container")
            .field("media_type()", &self.media_type())
            .field("data_files()", &self.data_files())
            .field("signatures()", &self.signatures())
            .finish_non_exhaustive()
    }
}
