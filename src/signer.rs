use crate::util::{
    ffi_variant_doc, library_and_ffi_variants_doc, library_variant_doc, path_to_str,
};
use crate::{
    Algorithm, AlgorithmSetting, DerCert, Error, ExposeSecret, ProfileSetting, SecretString,
    SigningProfile,
};
use cxx::UniquePtr;
use libdigidocpp_sys::signer::ffi;
use std::fmt::{Debug, Formatter};
use std::path::Path;
use std::pin::Pin;

mod sealed {
    use libdigidocpp_sys::signer::ffi;
    use std::pin::Pin;

    #[allow(unreachable_pub, reason = "sealed trait")]
    pub trait Sealed {
        fn ffi_signer(&self) -> &ffi::Signer;
        fn ffi_signer_mut(&mut self) -> Pin<&mut ffi::Signer>;
    }
}

/// Certificate and signature settings of a signer WITHOUT the ability to sign.
///
/// Container operations that add a time-stamp or OCSP response (e.g.
/// [`Container::prepare_signature`](crate::Container::prepare_signature)) need only this.
/// [`Signer`] adds signing.
///
/// Corresponds to the settings of upstream [`digidoc::Signer`].
///
/// [`digidoc::Signer`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signer.html
pub trait PartialSigner: sealed::Sealed {
    /// Returns the signing certificate, or `None` if the signer has none.
    ///
    /// [`Pkcs11Signer`] reads it from the inserted card on the first call.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn cert(&self) -> Result<Option<DerCert>, Error> {
        Ok(DerCert::from_ffi(ffi::cert(self.ffi_signer())?))
    }

    /// Returns the algorithm used to sign.
    ///
    /// Can be set with [`set_method`](Self::set_method). Otherwise it is chosen automatically by libdigidocpp.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn method(&self) -> Result<Algorithm, Error> {
        Ok(Algorithm::from(ffi::method(self.ffi_signer())?.as_str()))
    }

    /// Returns the signature profile the signer produces.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn profile(&self) -> Result<SigningProfile, Error> {
        Ok(SigningProfile::from(
            ffi::profile(self.ffi_signer())?.as_str(),
        ))
    }

    /// Returns the additional User-Agent info for time-stamp and OCSP requests.
    ///
    /// If not empty, libdigidocpp uses it in place of the one from
    /// [`initialize_with_user_agent`](crate::initialize_with_user_agent). See
    /// [`PartialSigner::set_user_agent`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn user_agent(&self) -> Result<String, Error> {
        Ok(ffi::user_agent(self.ffi_signer())?)
    }

    /// Returns whether signatures follow the XAdES EN standard.
    ///
    /// A street address in the production place needs this.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn using_en_profile(&self) -> Result<bool, Error> {
        Ok(ffi::using_en_profile(self.ffi_signer())?)
    }

    /// Returns the city of the signature production place.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn city(&self) -> Result<String, Error> {
        Ok(ffi::city(self.ffi_signer())?)
    }

    /// Returns the street address of the signature production place.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn street_address(&self) -> Result<String, Error> {
        Ok(ffi::street_address(self.ffi_signer())?)
    }

    /// Returns the state or province of the signature production place.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn state_or_province(&self) -> Result<String, Error> {
        Ok(ffi::state_or_province(self.ffi_signer())?)
    }

    /// Returns the postal code of the signature production place.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn postal_code(&self) -> Result<String, Error> {
        Ok(ffi::postal_code(self.ffi_signer())?)
    }

    /// Returns the country of the signature production place.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn country_name(&self) -> Result<String, Error> {
        Ok(ffi::country_name(self.ffi_signer())?)
    }

    /// Returns the signer roles written into signatures.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn signer_roles(&self) -> Result<Vec<String>, Error> {
        Ok(ffi::signer_roles(self.ffi_signer())?)
    }

    /// Sets the algorithm used to sign.
    ///
    /// [`AlgorithmSetting::Default`] restores the libdigidocpp default.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn set_method(&mut self, method: AlgorithmSetting) -> Result<(), Error> {
        Ok(ffi::set_method(self.ffi_signer_mut(), method.as_ref())?)
    }

    /// Sets the signature profile.
    ///
    /// [`ProfileSetting::Default`] restores the default profile.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn set_profile(&mut self, profile: ProfileSetting) -> Result<(), Error> {
        Ok(ffi::set_profile(self.ffi_signer_mut(), profile.as_ref())?)
    }

    /// Sets additional User-Agent info for time-stamp and OCSP requests.
    ///
    /// If not empty, libdigidocpp currently uses it in place of the one from
    /// [`initialize_with_user_agent`](crate::initialize_with_user_agent).
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn set_user_agent(&mut self, user_agent: &str) -> Result<(), Error> {
        Ok(ffi::set_user_agent(self.ffi_signer_mut(), user_agent)?)
    }

    /// Sets whether signatures follow the XAdES EN standard.
    ///
    /// Without it, libdigidocpp currently drops the street address of
    /// [`set_signature_production_place_v2`](Self::set_signature_production_place_v2).
    /// A street address there enables it.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn set_en_profile(&mut self, enable: bool) -> Result<(), Error> {
        Ok(ffi::set_en_profile(self.ffi_signer_mut(), enable)?)
    }

    /// Sets the signature production place written into signatures.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn set_signature_production_place(
        &mut self,
        city: &str,
        state_or_province: &str,
        postal_code: &str,
        country_name: &str,
    ) -> Result<(), Error> {
        Ok(ffi::set_signature_production_place(
            self.ffi_signer_mut(),
            city,
            state_or_province,
            postal_code,
            country_name,
        )?)
    }

    /// Sets the signature production place, with a street address.
    ///
    /// A non-empty `street_address` also enables the XAdES EN profile.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn set_signature_production_place_v2(
        &mut self,
        city: &str,
        street_address: &str,
        state_or_province: &str,
        postal_code: &str,
        country_name: &str,
    ) -> Result<(), Error> {
        Ok(ffi::set_signature_production_place_v2(
            self.ffi_signer_mut(),
            city,
            street_address,
            state_or_province,
            postal_code,
            country_name,
        )?)
    }

    /// Sets the signer roles written into signatures.
    ///
    /// libdigidocpp's manual states only one role must be used. When specifying resolution, it
    /// should be separated by a slash: `"role / resolution"`.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    fn set_signer_roles(&mut self, roles: &[String]) -> Result<(), Error> {
        Ok(ffi::set_signer_roles(
            self.ffi_signer_mut(),
            &roles.to_vec(),
        )?)
    }
}

/// A signer that holds a private key and signs digests with it.
///
/// [`Container::sign`](crate::Container::sign) needs this. It also has the certificate and
/// settings of [`PartialSigner`].
///
/// Corresponds to upstream [`digidoc::Signer`].
///
/// [`digidoc::Signer`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signer.html
pub trait Signer: PartialSigner {
    /// Signs `digest` and returns the signature value.
    ///
    /// `digest` must be computed with the hash function included in `method`.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, signing fails on a wrong or locked PIN (see
    /// [`ExceptionCode`](crate::ExceptionCode)).
    #[doc = ffi_variant_doc!()]
    fn sign(&self, method: Algorithm, digest: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(ffi::sign(self.ffi_signer(), method.as_ref(), digest)?)
    }
}

/// A partial signer that only carries a certificate and settings while the key lives elsewhere, so
/// it can't be used for signing directly.
///
/// Use it to prepare a signature that is signed externally. Intended use case is calling
/// [`SignatureMut::extend_signature_profile`](crate::SignatureMut::extend_signature_profile) and
/// [`Container::extend_container_validity`](crate::Container::extend_container_validity).
pub struct ExternalPartialSigner(UniquePtr<ffi::ExternalPartialSigner>);

impl ExternalPartialSigner {
    /// Creates a signer for the DER certificate `cert`.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when `cert` is not empty and not a valid certificate.
    #[doc = ffi_variant_doc!()]
    pub fn new(cert: impl Into<DerCert>) -> Result<Self, Error> {
        let cert: DerCert = cert.into();

        Ok(Self(ffi::make_external_partial_signer(cert.as_ref())?))
    }
}

impl sealed::Sealed for ExternalPartialSigner {
    fn ffi_signer(&self) -> &ffi::Signer {
        ffi::external_as_signer(&self.0)
    }

    fn ffi_signer_mut(&mut self) -> Pin<&mut ffi::Signer> {
        ffi::external_as_signer_mut(self.0.pin_mut())
    }
}

impl PartialSigner for ExternalPartialSigner {}

impl Debug for ExternalPartialSigner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternalPartialSigner")
            .finish_non_exhaustive()
    }
}

/// Signs with a smart card, such as an Estonian ID card, through a PKCS #11 driver.
///
/// It uses the signing certificate of the inserted card. Wraps upstream [`digidoc::PKCS11Signer`].
///
/// [`digidoc::PKCS11Signer`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1PKCS11Signer.html
pub struct Pkcs11Signer(UniquePtr<ffi::PKCS11Signer>);

impl Pkcs11Signer {
    /// Loads the driver from [`Conf::pkcs11_driver`](crate::Conf::pkcs11_driver).
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when the driver cannot be loaded.
    #[doc = ffi_variant_doc!()]
    pub fn new() -> Result<Self, Error> {
        Ok(Self(ffi::make_pkcs11_signer("")?))
    }

    /// Loads the PKCS #11 driver at `driver`, for example `/usr/lib/opensc-pkcs11.so`.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when the driver cannot be loaded.
    /// Returns [`Error::Internal`] when `driver` is empty or not valid UTF-8.
    #[doc = ffi_variant_doc!()]
    pub fn with_driver(driver: impl AsRef<Path>) -> Result<Self, Error> {
        let driver = path_to_str(driver.as_ref())?;
        if driver.is_empty() {
            return Err(Error::Internal(String::from("driver path is empty")));
        }
        Ok(Self(ffi::make_pkcs11_signer(driver)?))
    }

    /// Sets the PIN used to sign (PIN2 on Estonian ID cards).
    ///
    /// Not used when the reader has its own PIN pad. Signing fails with
    /// [`ExceptionCode::PINLocked`](crate::ExceptionCode::PINLocked) when the card is locked.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_pin(&mut self, pin: impl Into<SecretString>) -> Result<(), Error> {
        let pin = pin.into();

        Ok(ffi::pkcs11_set_pin(self.0.pin_mut(), pin.expose_secret())?)
    }
}

impl sealed::Sealed for Pkcs11Signer {
    fn ffi_signer(&self) -> &ffi::Signer {
        ffi::pkcs11_as_signer(&self.0)
    }

    fn ffi_signer_mut(&mut self) -> Pin<&mut ffi::Signer> {
        ffi::pkcs11_as_signer_mut(self.0.pin_mut())
    }
}

impl PartialSigner for Pkcs11Signer {}

impl Signer for Pkcs11Signer {}

impl Debug for Pkcs11Signer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pkcs11Signer").finish_non_exhaustive()
    }
}

/// Signs with a certificate from the Windows certificate store. Available on Windows only.
///
/// Wraps upstream [`digidoc::WinSigner`].
///
/// [`digidoc::WinSigner`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1WinSigner.html
#[cfg(windows)]
pub struct WinSigner(UniquePtr<ffi::WinSigner>);

#[cfg(windows)]
impl WinSigner {
    /// Creates a signer.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn new() -> Result<Self, Error> {
        Ok(Self(ffi::make_win_signer()?))
    }

    /// Sets the PIN that libdigidocpp passes to Windows when signing.
    ///
    /// Windows can still ask for the PIN.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_pin(&mut self, pin: impl Into<SecretString>) -> Result<(), Error> {
        let pin = pin.into();

        Ok(ffi::win_set_pin(self.0.pin_mut(), pin.expose_secret())?)
    }

    /// Sets whether to use the first suitable certificate, to skip the selection dialog.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_select_first(&mut self, select_first: bool) -> Result<(), Error> {
        Ok(ffi::win_set_select_first(self.0.pin_mut(), select_first)?)
    }

    /// Selects the certificate with this thumbprint, to skip the selection dialog.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_thumbprint(&mut self, thumbprint: &[u8]) -> Result<(), Error> {
        Ok(ffi::win_set_thumbprint(self.0.pin_mut(), thumbprint)?)
    }
}

#[cfg(windows)]
impl sealed::Sealed for WinSigner {
    fn ffi_signer(&self) -> &ffi::Signer {
        ffi::win_as_signer(&self.0)
    }

    fn ffi_signer_mut(&mut self) -> Pin<&mut ffi::Signer> {
        ffi::win_as_signer_mut(self.0.pin_mut())
    }
}

#[cfg(windows)]
impl PartialSigner for WinSigner {}

#[cfg(windows)]
impl Signer for WinSigner {}

#[cfg(windows)]
impl Debug for WinSigner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WinSigner").finish_non_exhaustive()
    }
}

/// Signs with the key and certificate from a PKCS #12 (`.p12`, `.pfx`) file.
///
/// Wraps upstream [`digidoc::PKCS12Signer`].
///
/// [`digidoc::PKCS12Signer`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1PKCS12Signer.html
pub struct Pkcs12Signer(UniquePtr<ffi::PKCS12Signer>);

impl Pkcs12Signer {
    /// Loads the PKCS #12 file at `path`, decrypted with `pass`.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when the password is wrong or the file is missing or
    /// invalid.
    /// Returns [`Error::Internal`] when `path` is not valid UTF-8.
    #[doc = ffi_variant_doc!()]
    pub fn new(path: impl AsRef<Path>, pass: impl Into<SecretString>) -> Result<Self, Error> {
        let path = path_to_str(path.as_ref())?;
        let pass = pass.into();

        Ok(Self(ffi::make_pkcs12_signer(path, pass.expose_secret())?))
    }
}

impl sealed::Sealed for Pkcs12Signer {
    fn ffi_signer(&self) -> &ffi::Signer {
        ffi::pkcs12_as_signer(&self.0)
    }

    fn ffi_signer_mut(&mut self) -> Pin<&mut ffi::Signer> {
        ffi::pkcs12_as_signer_mut(self.0.pin_mut())
    }
}

impl PartialSigner for Pkcs12Signer {}

impl Signer for Pkcs12Signer {}

impl Debug for Pkcs12Signer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pkcs12Signer").finish_non_exhaustive()
    }
}
