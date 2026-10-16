use crate::Error;
use crate::algorithm::DigestSetting;
use crate::stream::call_then_deliver_stream_failure;
use crate::util::{
    ffi_variant_doc, io_variant_doc, library_and_ffi_variants_doc, library_io_and_ffi_variants_doc,
    library_variant_doc, path_to_str, reader_or_writer_panic_doc,
};
use libdigidocpp_sys::data_file::ffi;
use libdigidocpp_sys::stream::Sink;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use std::path::Path;

/// A file inside a [`Container`](crate::Container).
///
/// Wraps upstream [`digidoc::DataFile`].
///
/// # Examples
///
/// ```no_run
/// use digidocpp::Container;
///
/// # fn main() -> Result<(), digidocpp::Error> {
/// let container = Container::open("document.asice")?;
/// for data_file in container.data_files()? {
///     data_file.save_as(data_file.file_name()?)?;
/// }
/// # Ok(())
/// # }
/// ```
///
/// [`digidoc::DataFile`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1DataFile.html
#[derive(Clone, Copy)]
pub struct DataFile<'a>(pub(crate) &'a ffi::DataFile);

impl DataFile<'_> {
    /// Returns the data file ID.
    ///
    /// For ASiC containers, it is the file name.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn id(&self) -> Result<String, Error> {
        Ok(ffi::id(self.0)?)
    }

    /// Returns the MIME type.
    ///
    /// ASiC-S containers do not store it, so libdigidocpp reports `application/octet-stream`.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn media_type(&self) -> Result<String, Error> {
        Ok(ffi::media_type(self.0)?)
    }

    /// Returns the file name inside the container.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn file_name(&self) -> Result<String, Error> {
        Ok(ffi::file_name(self.0)?)
    }

    /// Returns the size in bytes.
    ///
    /// # Errors
    #[doc = library_io_and_ffi_variants_doc!()]
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
    pub fn file_size(&self) -> Result<u64, Error> {
        call_then_deliver_stream_failure(|| ffi::file_size(self.0))
    }

    /// Calculates the digest of the content.
    ///
    /// [`DigestSetting::Default`] uses the hash function from
    /// [`Conf::digest_uri`](crate::conf::Conf::digest_uri).
    ///
    /// # Errors
    #[doc = library_io_and_ffi_variants_doc!()]
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
    pub fn calc_digest(&self, method: impl Into<DigestSetting>) -> Result<Vec<u8>, Error> {
        let method = method.into();

        call_then_deliver_stream_failure(|| ffi::calc_digest(self.0, method.as_ref()))
    }

    /// Writes the content to the file at `path`, overwriting it.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// Returns [`Error::Internal`] when `path` is not valid UTF-8.
    #[doc = concat!(io_variant_doc!(), ffi_variant_doc!())]
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
    pub fn save_as(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path_to_str(path.as_ref())?;
        call_then_deliver_stream_failure(|| ffi::save_as(self.0, path))
    }

    /// Writes the content to `writer`.
    ///
    /// # Errors
    #[doc = library_io_and_ffi_variants_doc!()]
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
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
