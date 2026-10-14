use crate::signature::{Signature, SignatureMut};
use crate::stream::call_then_deliver_stream_failure;
use crate::util::{
    ffi_variant_doc, io_variant_doc, library_and_ffi_variants_doc, library_variant_doc,
    path_to_str, reader_or_writer_panic_doc,
};
use crate::{DataFile, Error, PartialSigner, Signer};
use libdigidocpp_sys::container::{self, ffi};
use libdigidocpp_sys::stream::Source;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Seek};
use std::path::Path;
use strum::{AsRefStr, Display as StrumDisplay, EnumString};

/// Container format. String form (`ToString`/`FromStr`) is the media type.
#[derive(Clone, Debug, Eq, Hash, PartialEq, AsRefStr, StrumDisplay, EnumString)]
#[non_exhaustive]
pub enum ContainerType {
    /// ASiC-E (`.asice`).
    ///
    /// Used for new containers unless the extension is `.asics` or `.scs`. Holds several data
    /// files and signatures.
    #[strum(serialize = "application/vnd.etsi.asic-e+zip")]
    AsicE,
    /// ASiC-S (`.asics` or `.scs`): one data file with a time-stamp token. XAdES signatures in ASiC-S are
    /// read-only.
    #[strum(serialize = "application/vnd.etsi.asic-s+zip")]
    AsicS,
    /// Lithuanian ADOC. Read locally. It cannot be signed or saved.
    #[strum(serialize = "application/vnd.lt.archyvai.adoc-2008")]
    Adoc,
    /// Legacy DigiDoc (`.ddoc`). Validated by SiVa.
    #[strum(serialize = "application/x-ddoc")]
    Ddoc,
    /// Signed PDF. Validated by SiVa.
    #[strum(serialize = "application/pdf")]
    Pdf,
    /// A media type this crate does not know.
    #[strum(default, transparent)]
    Other(String),
}

/// A package of data files and the electronic signatures over them.
///
/// The format is a [`ContainerType`]. libdigidocpp creates and signs ASiC-E containers. It signs
/// ASiC-S containers only with a time-stamp token. It reads and validates the other formats.
///
/// Data files can be added and removed only while the container has no signatures.
///
/// Wraps upstream [`digidoc::Container`].
///
/// # Examples
///
/// Sign a file with an ID card through PKCS #11. With the default profile, signing contacts the
/// time-stamp and OCSP services set in [`Conf`](crate::Conf).
///
/// ```no_run
/// use digidocpp::{Container, Pkcs11Signer};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// digidocpp::initialize()?;
///
/// let mut container = Container::create("contract.asice")?;
/// container.add_data_file("contract.pdf", "application/pdf")?;
///
/// let mut signer = Pkcs11Signer::new()?;
/// signer.set_pin(std::env::var("PIN2")?)?;
/// container.sign(&mut signer)?;
/// container.save()?;
/// # Ok(())
/// # }
/// ```
///
/// Check the signatures of a container. A signature with warnings or one that is valid but not
/// qualified counts as valid.
///
/// ```no_run
/// use digidocpp::{Container, Error, Signature, ValidatorStatus};
///
/// # fn main() -> Result<(), Error> {
/// digidocpp::initialize()?;
///
/// let is_valid = |signature: Signature<'_>| -> Result<bool, Error> {
///     Ok(matches!(
///         signature.validator_result()?.status(),
///         Some(ValidatorStatus::Valid | ValidatorStatus::Warning | ValidatorStatus::NonQSCD)
///     ))
/// };
///
/// let container = Container::open("contract.asice")?;
/// for signature in container.signatures()? {
///     println!("{}", is_valid(signature)?);
/// }
/// # Ok(())
/// # }
/// ```
///
/// [`digidoc::Container`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Container.html
pub struct Container(cxx::UniquePtr<ffi::Container>);

/// Outcome of [`Container::extend_container_validity`].
#[derive(Debug)]
#[expect(clippy::exhaustive_enums)]
pub enum ValidityExtension {
    /// Signatures were extended inside the original container.
    InPlace {
        /// Number of signatures extended.
        extended_count: usize,
    },
    /// The original container could not be extended.
    ///
    /// It is now the data file of a new ASiC-S container which carries a time-stamp. The new
    /// container is not saved yet. Its path is currently the original path plus `.asics`.
    Wrapped(Container),
}

impl Container {
    /// Opens the container at `path`.
    ///
    /// The format is detected from the extension and content.
    ///
    /// ASiC-E, ASiC-S and ADOC are read locally. DDOC, PDF and ASiC-E or ASiC-S with CAdES
    /// signatures are sent to the SiVa service
    /// ([`Conf::verify_service_uri`](crate::Conf::verify_service_uri)). To disallow SiVa service,
    /// use [`Container::open_requiring_online_validation_consent`] with a callback returning false.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when the file is missing or not a supported container.
    /// Returns [`Error::Internal`] when `path` is not valid UTF-8.
    #[doc = ffi_variant_doc!()]
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path_to_str(path.as_ref())?;
        Ok(Self(ffi::open(path)?))
    }

    /// Opens a container at `path` and asks `consent` before online validation.
    ///
    /// Calls `consent` before libdigidocpp sends the container (or its hashes) to the SiVa service.
    /// `consent` is not called for formats that libdigidocpp validates locally. Container format is
    /// detected from the extension and content.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp throws when `consent` returns false, when the file is missing or
    /// when the file is not a supported container.
    /// Returns [`Error::Internal`] when `path` is not valid UTF-8.
    #[doc = ffi_variant_doc!()]
    pub fn open_requiring_online_validation_consent(
        path: impl AsRef<Path>,
        mut consent: impl FnMut() -> bool,
    ) -> Result<Self, Error> {
        let path = path_to_str(path.as_ref())?;
        Ok(Self(container::open_with_callback_resuming_panic(
            path,
            &mut consent,
        )?))
    }

    /// Creates an empty container that will be saved at `path`.
    ///
    /// The container format is determined by the file extension. `.asics` or `.scs` creates ASiC-S,
    /// anything else ASiC-E. The file is written after calling [`Container::save`].
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// Returns [`Error::Internal`] when `path` is not valid UTF-8.
    #[doc = ffi_variant_doc!()]
    pub fn create(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path_to_str(path.as_ref())?;
        Ok(Self(ffi::create(path)?))
    }

    /// Returns the container format.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn media_type(&self) -> Result<ContainerType, Error> {
        Ok(ContainerType::from(ffi::media_type(&self.0)?.as_str()))
    }

    /// Returns the data files, in container order.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn data_files(&self) -> Result<Vec<DataFile<'_>>, Error> {
        let count = ffi::data_files_size(&self.0)?;
        let data_files = (0..count)
            .map(|index| ffi::data_file_at(&self.0, index).map(DataFile))
            .collect::<Result<_, _>>()?;

        Ok(data_files)
    }

    /// Adds the file at `path` as a data file, named after its file name.
    ///
    /// `media_type` is a MIME type such as `text/plain`.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when:
    /// - the container has signatures
    /// - the file is missing or empty
    /// - a data file with the same name exists or the name is invalid
    /// - `media_type` is not a valid MIME type
    ///
    /// Returns [`Error::Internal`] when `path` is not valid UTF-8.
    #[doc = ffi_variant_doc!()]
    pub fn add_data_file(&mut self, path: impl AsRef<Path>, media_type: &str) -> Result<(), Error> {
        let path = path_to_str(path.as_ref())?;
        ffi::add_data_file(self.0.pin_mut(), path, media_type)?;
        Ok(())
    }

    /// Adds data read from `reader` as a data file called `name`.
    ///
    /// libdigidocpp reads `reader` again each time it digests or saves the file, so the reader
    /// must stay usable for the container's lifetime. A reader error surfaces as [`Error::Io`]
    /// from the method that triggers the read, such as [`Container::save`]. A reader panic
    /// is resumed by that method too.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when:
    /// - the container has signatures
    /// - a data file with the same name exists or the name is invalid
    /// - `media_type` is not a valid MIME type
    ///
    #[doc = ffi_variant_doc!()]
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

    /// Removes the data file at `index`.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when the container has signatures or `index` is out of
    /// range.
    /// Returns [`Error::Internal`] when `index` does not fit in 32 bits.
    #[doc = ffi_variant_doc!()]
    pub fn remove_data_file(&mut self, index: usize) -> Result<(), Error> {
        let index = index_as_u32(index, "data file")?;
        ffi::remove_data_file(self.0.pin_mut(), index)?;
        Ok(())
    }

    /// Saves the container to the path it was opened or created with or last saved to.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when the container has no data files, its format cannot be
    /// saved (such as ASiC-S with XAdES signatures) or the file cannot be created.
    #[doc = concat!(io_variant_doc!(), ffi_variant_doc!())]
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
    pub fn save(&mut self) -> Result<(), Error> {
        call_then_deliver_stream_failure(|| ffi::save(self.0.pin_mut(), ""))
    }

    /// Saves the container to `path`.
    ///
    /// Later [`Container::save`] calls use `path` too.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when the container has no data files, its format cannot be
    /// saved or the file cannot be created.
    /// Returns [`Error::Internal`] when `path` is empty or not valid UTF-8.
    #[doc = concat!(io_variant_doc!(), ffi_variant_doc!())]
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path_to_str(path.as_ref())?;
        if path.is_empty() {
            return Err(Error::Internal(String::from("path is empty")));
        }
        call_then_deliver_stream_failure(|| ffi::save(self.0.pin_mut(), path))
    }

    /// Adds the signatures in `xades_signatures`, the bytes of an [`asic:XAdESSignatures`] XML
    /// document.
    ///
    /// ASiC-E only.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when the container has no data files, is not ASiC-E or
    /// `xades_signatures` does not parse.
    #[doc = ffi_variant_doc!()]
    ///
    /// [`asic:XAdESSignatures`]: https://www.etsi.org/deliver/etsi_en/319100_319199/31916201/01.01.01_60/en_31916201v010101p.pdf
    pub fn add_ades_signature(&mut self, xades_signatures: &[u8]) -> Result<(), Error> {
        Ok(ffi::add_ades_signature(self.0.pin_mut(), xades_signatures)?)
    }

    /// Returns the signatures, in container order.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn signatures(&self) -> Result<Vec<Signature<'_>>, Error> {
        let count = ffi::signatures_size(&self.0)?;
        let data_files = (0..count)
            .map(|index| ffi::signature_at(&self.0, index).map(Signature))
            .collect::<Result<_, _>>()?;

        Ok(data_files)
    }

    /// Returns the signature at `index` for modification.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when `index` is out of range.
    #[doc = ffi_variant_doc!()]
    pub fn signature_mut(&mut self, index: usize) -> Result<SignatureMut<'_>, Error> {
        Ok(SignatureMut(ffi::signature_at_mut(
            self.0.pin_mut(),
            index,
        )?))
    }

    /// Removes the signature at `index`.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when `index` is out of range.
    /// Returns [`Error::Internal`] when `index` does not fit in 32 bits.
    #[doc = ffi_variant_doc!()]
    pub fn remove_signature(&mut self, index: usize) -> Result<(), Error> {
        let index = index_as_u32(index, "signature")?;
        ffi::remove_signature(self.0.pin_mut(), index)?;
        Ok(())
    }

    /// Signs the data files with `signer` and adds the signature.
    ///
    /// The signer's [profile](PartialSigner::profile) sets what libdigidocpp adds after signing.
    /// Profiles with `time-stamp` add a time-stamp and an OCSP response. For this,
    /// libdigidocpp contacts the services from [`Conf`](crate::Conf). ASiC-S accepts one
    /// signature and only with profile
    /// [`ProfileSetting::TimeStampToken`](crate::ProfileSetting::TimeStampToken).
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when:
    /// - the container has no data files or its format cannot be signed
    /// - signing fails, such as on a wrong PIN
    /// - a service request fails
    ///
    #[doc = concat!(io_variant_doc!(), ffi_variant_doc!())]
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
    pub fn sign<S>(&mut self, signer: &mut S) -> Result<SignatureMut<'_>, Error>
    where
        S: Signer + ?Sized,
    {
        let container = self.0.pin_mut();
        let signer = signer.ffi_signer_mut();

        call_then_deliver_stream_failure(move || ffi::sign(container, signer)).map(SignatureMut)
    }

    /// Extends the validity of the signatures in this container with a new time-stamp.
    ///
    /// Where possible, libdigidocpp extends the signatures in place
    /// ([`ValidityExtension::InPlace`]). Otherwise it wraps the container in a new ASiC-S
    /// container ([`ValidityExtension::Wrapped`]).
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when:
    /// - the container has no signatures
    /// - no signature has an Estonian signing certificate
    /// - every considered signature has a `time-stamp` profile and no OCSP response
    ///
    #[doc = concat!(io_variant_doc!(), ffi_variant_doc!())]
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
    pub fn extend_container_validity<S>(
        &mut self,
        signer: &mut S,
    ) -> Result<ValidityExtension, Error>
    where
        S: PartialSigner + ?Sized,
    {
        let container = self.0.pin_mut();
        let signer = signer.ffi_signer_mut();

        let extended = call_then_deliver_stream_failure(move || {
            ffi::extend_container_validity(container, signer)
        })?;

        Ok(if extended.container.is_null() {
            ValidityExtension::InPlace {
                extended_count: extended.extended_count,
            }
        } else {
            ValidityExtension::Wrapped(Self(extended.container))
        })
    }

    /// Adds a signature entry WITHOUT a signature value, for signing OUTSIDE libdigidocpp.
    ///
    /// `signer` supplies the certificate and signature settings. Then:
    /// 1. Sign [`Signature::data_to_sign`] with the key of the certificate (e.g. externally).
    /// 2. Pass the result to [`SignatureMut::set_signature_value`].
    /// 3. Call [`SignatureMut::extend_signature_profile`] with the same signer, to add the
    ///    time-stamp and OCSP response of its profile, if it needs them.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use digidocpp::{Container, ExternalPartialSigner};
    ///
    /// # fn sign_remotely(_digest: &[u8]) -> Vec<u8> { Vec::new() }
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # digidocpp::initialize()?;
    /// let mut container = Container::open("unsigned.asice")?;
    ///
    /// let mut signer = ExternalPartialSigner::new(std::fs::read("signer.der")?)?;
    ///
    /// let mut signature = container.prepare_signature(&mut signer)?;
    /// let value = sign_remotely(&signature.as_signature().data_to_sign()?);
    /// signature.set_signature_value(&value)?;
    /// signature.extend_signature_profile(&mut signer)?;
    ///
    /// container.save()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when the container has no data files or is not ASiC-E.
    #[doc = concat!(io_variant_doc!(), ffi_variant_doc!())]
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
    pub fn prepare_signature<S>(&mut self, signer: &mut S) -> Result<SignatureMut<'_>, Error>
    where
        S: PartialSigner + ?Sized,
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
        let media_type = self.media_type().map_or_else(
            |_| String::from("<unknown>"),
            |media_type| media_type.to_string(),
        );
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
