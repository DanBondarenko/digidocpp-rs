use crate::Error;
use crate::stream::call_then_deliver_stream_failure;
use crate::util::path_to_str;
use libdigidocpp_sys::data_file::ffi;
use libdigidocpp_sys::stream::Sink;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
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
        call_then_deliver_stream_failure(|| ffi::file_size(self.0))
    }

    pub fn calc_digest(&self, method: &str) -> Result<Vec<u8>, Error> {
        call_then_deliver_stream_failure(|| ffi::calc_digest(self.0, method))
    }

    pub fn save_as(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path_to_str(path.as_ref())?;
        call_then_deliver_stream_failure(|| ffi::save_as(self.0, path))
    }

    pub fn write_to(&self, mut writer: impl Write) -> Result<(), Error> {
        let mut sink = Sink::new(&mut writer);
        call_then_deliver_stream_failure(|| ffi::write_to(self.0, &mut sink))
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
