use libdigidocpp_sys::data_file::ffi;
use libdigidocpp_sys::exception::FfiException;

pub struct DataFile<'a>(pub(crate) &'a ffi::DataFile);

impl DataFile<'_> {
    pub fn id(&self) -> Result<String, FfiException> {
        Ok(ffi::id(self.0)?)
    }

    pub fn file_name(&self) -> Result<String, FfiException> {
        Ok(ffi::file_name(self.0)?)
    }
    pub fn file_size(&self) -> Result<u64, FfiException> {
        Ok(ffi::file_size(self.0)?)
    }
    pub fn calc_digest(&self, method: &str) -> Result<Vec<u8>, FfiException> {
        Ok(ffi::calc_digest(self.0, method)?)
    }
}
