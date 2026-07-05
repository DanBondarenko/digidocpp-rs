use crate::Error;
use crate::util::path_to_str;
use libdigidocpp_sys::data_file::ffi;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;

pub struct DataFile<'a>(pub(crate) &'a ffi::DataFile);

impl DataFile<'_> {
    pub fn id(&self) -> Result<String, Error> {
        Ok(ffi::id(self.0)?)
    }

    pub fn media_type(&self) -> Result<String, Error> {
        Ok(ffi::media_type(self.0)?)
    }

    pub fn file_name(&self) -> Result<String, Error> {
        Ok(ffi::file_name(self.0)?)
    }

    pub fn file_size(&self) -> Result<u64, Error> {
        Ok(ffi::file_size(self.0)?)
    }

    pub fn calc_digest(&self, method: &str) -> Result<Vec<u8>, Error> {
        Ok(ffi::calc_digest(self.0, method)?)
    }

    pub fn save_as(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path_to_str(path.as_ref())?;
        Ok(ffi::save_as(self.0, path)?)
    }
}

impl Display for DataFile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let file_name = self
            .file_name()
            .unwrap_or_else(|_| String::from("<file name unavailable>"));
        let file_size = self.file_size().ok();

        if let Some(size) = file_size {
            write!(f, "{file_name} ({size} bytes)")
        } else {
            write!(f, "{file_name}")
        }
    }
}

impl Debug for DataFile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataFile")
            .field("id()", &self.id())
            .field("file_name()", &self.file_name())
            .field("file_size()", &self.file_size())
            .field("media_type()", &self.media_type())
            .finish_non_exhaustive()
    }
}
