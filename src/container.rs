use crate::signature::{Signature, SignatureMut};
use crate::stream::call_then_deliver_stream_failure;
use crate::util::path_to_str;
use crate::{DataFile, Error, Signer};
use libdigidocpp_sys::container::ffi;
use libdigidocpp_sys::stream::Source;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Seek};
use std::path::Path;

pub struct Container(cxx::UniquePtr<ffi::Container>);

#[derive(Debug)]
#[expect(clippy::exhaustive_enums)]
pub enum ValidityExtension {
    InPlace { extended_count: usize },
    Wrapped(Container),
}

impl Container {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path_to_str(path.as_ref())?;
        Ok(Self(ffi::open(path)?))
    }

    pub fn create(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path_to_str(path.as_ref())?;
        Ok(Self(ffi::create(path)?))
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

    pub fn add_data_file_from_reader(
        &mut self,
        name: &str,
        media_type: &str,
        reader: impl Read + Seek + 'static,
    ) -> Result<(), Error> {
        ffi::add_data_file_from_reader(
            self.0.pin_mut(),
            name,
            media_type,
            Box::new(Source::new(reader)),
        )?;
        Ok(())
    }

    pub fn remove_data_file(&mut self, index: usize) -> Result<(), Error> {
        let index = index_as_u32(index, "data file")?;
        ffi::remove_data_file(self.0.pin_mut(), index)?;
        Ok(())
    }

    // reuse path container opened with
    pub fn save(&mut self) -> Result<(), Error> {
        call_then_deliver_stream_failure(|| ffi::save(self.0.pin_mut(), ""))
    }

    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path_to_str(path.as_ref())?;
        if path.is_empty() {
            return Err(Error::Internal(String::from("path is empty")));
        }
        call_then_deliver_stream_failure(|| ffi::save(self.0.pin_mut(), path))
    }

    pub fn add_ades_signature(&mut self, signature: &[u8]) -> Result<(), Error> {
        Ok(ffi::add_ades_signature(self.0.pin_mut(), signature)?)
    }

    pub fn signatures(&self) -> Result<Vec<Signature<'_>>, Error> {
        let count = ffi::signatures_size(&self.0)?;
        let data_files = (0..count)
            .map(|index| ffi::signature_at(&self.0, index).map(Signature))
            .collect::<Result<_, _>>()?;

        Ok(data_files)
    }

    pub fn signature_mut(&mut self, index: usize) -> Result<SignatureMut<'_>, Error> {
        Ok(SignatureMut(ffi::signature_at_mut(
            self.0.pin_mut(),
            index,
        )?))
    }

    pub fn remove_signature(&mut self, index: usize) -> Result<(), Error> {
        let index = index_as_u32(index, "signature")?;
        ffi::remove_signature(self.0.pin_mut(), index)?;
        Ok(())
    }

    pub fn sign<S>(&mut self, signer: &mut S) -> Result<SignatureMut<'_>, Error>
    where
        S: Signer + ?Sized,
    {
        let container = self.0.pin_mut();
        let signer = signer.ffi_signer_mut();

        call_then_deliver_stream_failure(move || ffi::sign(container, signer)).map(SignatureMut)
    }

    pub fn extend_container_validity<S>(
        doc: &mut Self,
        signer: &mut S,
    ) -> Result<ValidityExtension, Error>
    where
        S: Signer + ?Sized,
    {
        let doc = doc.0.pin_mut();
        let signer = signer.ffi_signer_mut();

        let extended =
            call_then_deliver_stream_failure(move || ffi::extend_container_validity(doc, signer))?;

        Ok(if extended.container.is_null() {
            ValidityExtension::InPlace {
                extended_count: extended.extended_count,
            }
        } else {
            ValidityExtension::Wrapped(Self(extended.container))
        })
    }

    pub fn prepare_signature<S>(&mut self, signer: &mut S) -> Result<SignatureMut<'_>, Error>
    where
        S: Signer + ?Sized,
    {
        let container = self.0.pin_mut();
        let signer = signer.ffi_signer_mut();

        call_then_deliver_stream_failure(move || ffi::prepare_signature(container, signer))
            .map(SignatureMut)
    }
}

fn index_as_u32(index: usize, kind: &str) -> Result<u32, Error> {
    u32::try_from(index)
        .map_err(|_| Error::Internal(format!("{kind} index does not fit in 32 bits: {index}")))
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
