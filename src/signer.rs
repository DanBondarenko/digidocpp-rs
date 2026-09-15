use crate::util::path_to_str;
use crate::{DerCert, Error, ExposeSecret, SecretString};
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

pub trait Signer: sealed::Sealed {
    fn cert(&self) -> Result<Option<DerCert>, Error> {
        Ok(DerCert::from_ffi(ffi::cert(self.ffi_signer())?))
    }

    fn sign(&self, method: &str, digest: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(ffi::sign(self.ffi_signer(), method, digest)?)
    }

    fn method(&self) -> Result<String, Error> {
        Ok(ffi::method(self.ffi_signer())?)
    }

    fn profile(&self) -> Result<String, Error> {
        Ok(ffi::profile(self.ffi_signer())?)
    }

    fn user_agent(&self) -> Result<String, Error> {
        Ok(ffi::user_agent(self.ffi_signer())?)
    }

    fn using_en_profile(&self) -> Result<bool, Error> {
        Ok(ffi::using_en_profile(self.ffi_signer())?)
    }

    fn city(&self) -> Result<String, Error> {
        Ok(ffi::city(self.ffi_signer())?)
    }

    fn street_address(&self) -> Result<String, Error> {
        Ok(ffi::street_address(self.ffi_signer())?)
    }

    fn state_or_province(&self) -> Result<String, Error> {
        Ok(ffi::state_or_province(self.ffi_signer())?)
    }

    fn postal_code(&self) -> Result<String, Error> {
        Ok(ffi::postal_code(self.ffi_signer())?)
    }

    fn country_name(&self) -> Result<String, Error> {
        Ok(ffi::country_name(self.ffi_signer())?)
    }

    fn signer_roles(&self) -> Result<Vec<String>, Error> {
        Ok(ffi::signer_roles(self.ffi_signer())?)
    }

    fn set_method(&mut self, method: &str) -> Result<(), Error> {
        Ok(ffi::set_method(self.ffi_signer_mut(), method)?)
    }

    fn set_profile(&mut self, profile: &str) -> Result<(), Error> {
        Ok(ffi::set_profile(self.ffi_signer_mut(), profile)?)
    }

    fn set_user_agent(&mut self, user_agent: &str) -> Result<(), Error> {
        Ok(ffi::set_user_agent(self.ffi_signer_mut(), user_agent)?)
    }

    fn set_en_profile(&mut self, enable: bool) -> Result<(), Error> {
        Ok(ffi::set_en_profile(self.ffi_signer_mut(), enable)?)
    }

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

    fn set_signer_roles(&mut self, roles: &[String]) -> Result<(), Error> {
        Ok(ffi::set_signer_roles(
            self.ffi_signer_mut(),
            &roles.to_vec(),
        )?)
    }
}

pub struct Pkcs11Signer(UniquePtr<ffi::PKCS11Signer>);

impl Pkcs11Signer {
    pub fn new() -> Result<Self, Error> {
        Ok(Self(ffi::make_pkcs11_signer("")?))
    }

    pub fn with_driver(driver: impl AsRef<Path>) -> Result<Self, Error> {
        let driver = path_to_str(driver.as_ref())?;
        if driver.is_empty() {
            return Err(Error::Internal(String::from("driver path is empty")));
        }
        Ok(Self(ffi::make_pkcs11_signer(driver)?))
    }

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

impl Signer for Pkcs11Signer {}

impl Debug for Pkcs11Signer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pkcs11Signer").finish_non_exhaustive()
    }
}

#[cfg(windows)]
pub struct WinSigner(UniquePtr<ffi::WinSigner>);

#[cfg(windows)]
impl WinSigner {
    pub fn new() -> Result<Self, Error> {
        Ok(Self(ffi::make_win_signer()?))
    }

    pub fn set_pin(&mut self, pin: impl Into<SecretString>) -> Result<(), Error> {
        let pin = pin.into();

        Ok(ffi::win_set_pin(self.0.pin_mut(), pin.expose_secret())?)
    }

    pub fn set_select_first(&mut self, select_first: bool) -> Result<(), Error> {
        Ok(ffi::win_set_select_first(self.0.pin_mut(), select_first)?)
    }

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
impl Signer for WinSigner {}

#[cfg(windows)]
impl Debug for WinSigner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WinSigner").finish_non_exhaustive()
    }
}

pub struct Pkcs12Signer(UniquePtr<ffi::PKCS12Signer>);

impl Pkcs12Signer {
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

impl Signer for Pkcs12Signer {}

impl Debug for Pkcs12Signer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pkcs12Signer").finish_non_exhaustive()
    }
}
