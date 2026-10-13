//! Signers: upstream [`digidoc::Signer`], bound as [`ffi::Signer`].
//!
//! [`digidoc::Signer`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signer.html

use std::fmt::{self, Debug, Formatter};

/// Bridge for signers.
///
/// Concrete signers have `*_as_signer` functions that upcast to base [`Signer`], which the
/// container and signer-setting functions take.
#[cxx::bridge]
#[allow(clippy::missing_errors_doc, reason = "cxx")]
pub mod ffi {
    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("digidocpp/crypto/Signer.h");
        include!("digidocpp/crypto/PKCS11Signer.h");
        include!("digidocpp/crypto/PKCS12Signer.h");

        /// Upstream [`digidoc::Signer`], the base class of all signers.
        ///
        /// [`digidoc::Signer`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signer.html
        type Signer;

        /// Upstream [`digidoc::PKCS11Signer`].
        ///
        /// [`digidoc::PKCS11Signer`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1PKCS11Signer.html
        type PKCS11Signer;

        /// Upstream [`digidoc::PKCS12Signer`].
        ///
        /// [`digidoc::PKCS12Signer`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1PKCS12Signer.html
        type PKCS12Signer;
    }

    #[cfg(windows)]
    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("digidocpp/crypto/WinSigner.h");

        /// Upstream [`digidoc::WinSigner`], Windows only.
        ///
        /// [`digidoc::WinSigner`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1WinSigner.html
        type WinSigner;
    }

    #[cfg(windows)]
    #[namespace = "digidoc_rs::signer"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/signer_ffi.h");

        /// Constructs a [`WinSigner`].
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn make_win_signer() -> Result<UniquePtr<WinSigner>>;

        /// Calls upstream `WinSigner::setPin`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn win_set_pin(signer: Pin<&mut WinSigner>, pin: &str) -> Result<()>;

        /// Calls upstream `WinSigner::setSelectFirst`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn win_set_select_first(signer: Pin<&mut WinSigner>, select_first: bool) -> Result<()>;

        /// Calls upstream `WinSigner::setThumbprint`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn win_set_thumbprint(signer: Pin<&mut WinSigner>, thumbprint: &[u8]) -> Result<()>;

        /// Upcasts to [`Signer`].
        #[must_use]
        fn win_as_signer(signer: &WinSigner) -> &Signer;

        /// Upcasts to [`Signer`], mutable.
        #[must_use]
        fn win_as_signer_mut(signer: Pin<&mut WinSigner>) -> Pin<&mut Signer>;
    }

    #[namespace = "digidoc_rs::signer"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/signer_ffi.h");
        include!("libdigidocpp-sys/include/exception_ffi.h");

        /// Constructs a [`PKCS11Signer`].
        ///
        /// An empty `driver` means upstream `Conf::PKCS11Driver`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when the driver cannot
        /// be loaded.
        fn make_pkcs11_signer(driver: &str) -> Result<UniquePtr<PKCS11Signer>>;

        /// Calls upstream `PKCS11Signer::setPin`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn pkcs11_set_pin(signer: Pin<&mut PKCS11Signer>, pin: &str) -> Result<()>;

        /// Upcasts to [`Signer`].
        #[must_use]
        fn pkcs11_as_signer(signer: &PKCS11Signer) -> &Signer;

        /// Upcasts to [`Signer`], mutable.
        #[must_use]
        fn pkcs11_as_signer_mut(signer: Pin<&mut PKCS11Signer>) -> Pin<&mut Signer>;

        /// Constructs a [`PKCS12Signer`].
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when the file is
        /// missing or invalid, or the password is wrong.
        fn make_pkcs12_signer(path: &str, pass: &str) -> Result<UniquePtr<PKCS12Signer>>;

        /// Upcasts to [`Signer`].
        #[must_use]
        fn pkcs12_as_signer(signer: &PKCS12Signer) -> &Signer;

        /// Upcasts to [`Signer`], mutable.
        #[must_use]
        fn pkcs12_as_signer_mut(signer: Pin<&mut PKCS12Signer>) -> Pin<&mut Signer>;

        /// A [`Signer`] that only carries a certificate. Its `sign` always throws.
        type ExternalPartialSigner;

        /// Constructs an [`ExternalPartialSigner`].
        ///
        /// An empty `der` means no certificate.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when `der` is not a
        /// valid certificate.
        fn make_external_partial_signer(der: &[u8]) -> Result<UniquePtr<ExternalPartialSigner>>;

        /// Upcasts to [`Signer`].
        #[must_use]
        fn external_as_signer(signer: &ExternalPartialSigner) -> &Signer;

        /// Upcasts to [`Signer`], mutable.
        #[must_use]
        fn external_as_signer_mut(signer: Pin<&mut ExternalPartialSigner>) -> Pin<&mut Signer>;

        /// Calls upstream `Signer::cert`.
        ///
        /// Returns DER, or empty if absent.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn cert(signer: &Signer) -> Result<Vec<u8>>;

        /// Calls upstream `Signer::sign`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when signing fails on a
        /// wrong PIN.
        fn sign(signer: &Signer, method: &str, digest: &[u8]) -> Result<Vec<u8>>;

        /// Calls upstream `Signer::method`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn method(signer: &Signer) -> Result<String>;

        /// Calls upstream `Signer::profile`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn profile(signer: &Signer) -> Result<String>;

        /// Calls upstream `Signer::userAgent`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn user_agent(signer: &Signer) -> Result<String>;

        /// Calls upstream `Signer::usingENProfile`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn using_en_profile(signer: &Signer) -> Result<bool>;

        /// Calls upstream `Signer::city`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn city(signer: &Signer) -> Result<String>;

        /// Calls upstream `Signer::streetAddress`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn street_address(signer: &Signer) -> Result<String>;

        /// Calls upstream `Signer::stateOrProvince`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn state_or_province(signer: &Signer) -> Result<String>;

        /// Calls upstream `Signer::postalCode`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn postal_code(signer: &Signer) -> Result<String>;

        /// Calls upstream `Signer::countryName`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn country_name(signer: &Signer) -> Result<String>;

        /// Calls upstream `Signer::signerRoles`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn signer_roles(signer: &Signer) -> Result<Vec<String>>;

        /// Calls upstream `Signer::setMethod`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_method(signer: Pin<&mut Signer>, method: &str) -> Result<()>;

        /// Calls upstream `Signer::setProfile`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when the profile is
        /// unsupported.
        fn set_profile(signer: Pin<&mut Signer>, profile: &str) -> Result<()>;

        /// Calls upstream `Signer::setUserAgent`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_user_agent(signer: Pin<&mut Signer>, user_agent: &str) -> Result<()>;

        /// Calls upstream `Signer::setENProfile`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_en_profile(signer: Pin<&mut Signer>, enable: bool) -> Result<()>;

        /// Calls upstream `Signer::setSignatureProductionPlace`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_signature_production_place(
            signer: Pin<&mut Signer>,
            city: &str,
            state_or_province: &str,
            postal_code: &str,
            country_name: &str,
        ) -> Result<()>;

        /// Calls upstream `Signer::setSignatureProductionPlaceV2`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_signature_production_place_v2(
            signer: Pin<&mut Signer>,
            city: &str,
            street_address: &str,
            state_or_province: &str,
            postal_code: &str,
            country_name: &str,
        ) -> Result<()>;

        /// Calls upstream `Signer::setSignerRoles`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_signer_roles(signer: Pin<&mut Signer>, roles: &Vec<String>) -> Result<()>;
    }
}

impl Debug for ffi::Signer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Signer").finish_non_exhaustive()
    }
}

impl Debug for ffi::PKCS11Signer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PKCS11Signer").finish_non_exhaustive()
    }
}

impl Debug for ffi::PKCS12Signer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PKCS12Signer").finish_non_exhaustive()
    }
}

impl Debug for ffi::ExternalPartialSigner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExternalPartialSigner")
            .finish_non_exhaustive()
    }
}

#[cfg(windows)]
impl Debug for ffi::WinSigner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("WinSigner").finish_non_exhaustive()
    }
}
