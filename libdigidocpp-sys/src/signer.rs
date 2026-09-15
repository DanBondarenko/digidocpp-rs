use std::fmt::{self, Debug, Formatter};

#[cxx::bridge]
pub mod ffi {
    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("digidocpp/crypto/Signer.h");
        include!("digidocpp/crypto/PKCS11Signer.h");
        include!("digidocpp/crypto/PKCS12Signer.h");

        type Signer;
        type PKCS11Signer;
        type PKCS12Signer;
    }

    #[cfg(windows)]
    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("digidocpp/crypto/WinSigner.h");

        type WinSigner;
    }

    #[cfg(windows)]
    #[namespace = "digidoc_rs::signer"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/signer_ffi.h");

        fn make_win_signer() -> Result<UniquePtr<WinSigner>>;
        fn win_set_pin(signer: Pin<&mut WinSigner>, pin: &str) -> Result<()>;
        fn win_set_select_first(signer: Pin<&mut WinSigner>, select_first: bool) -> Result<()>;
        fn win_set_thumbprint(signer: Pin<&mut WinSigner>, thumbprint: &[u8]) -> Result<()>;
        #[must_use]
        fn win_as_signer(signer: &WinSigner) -> &Signer;
        #[must_use]
        fn win_as_signer_mut(signer: Pin<&mut WinSigner>) -> Pin<&mut Signer>;
    }

    #[namespace = "digidoc_rs::signer"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/signer_ffi.h");
        include!("libdigidocpp-sys/include/exception_ffi.h");

        fn make_pkcs11_signer(driver: &str) -> Result<UniquePtr<PKCS11Signer>>;
        fn pkcs11_set_pin(signer: Pin<&mut PKCS11Signer>, pin: &str) -> Result<()>;
        #[must_use]
        fn pkcs11_as_signer(signer: &PKCS11Signer) -> &Signer;
        #[must_use]
        fn pkcs11_as_signer_mut(signer: Pin<&mut PKCS11Signer>) -> Pin<&mut Signer>;

        fn make_pkcs12_signer(path: &str, pass: &str) -> Result<UniquePtr<PKCS12Signer>>;
        #[must_use]
        fn pkcs12_as_signer(signer: &PKCS12Signer) -> &Signer;
        #[must_use]
        fn pkcs12_as_signer_mut(signer: Pin<&mut PKCS12Signer>) -> Pin<&mut Signer>;

        fn cert(signer: &Signer) -> Result<Vec<u8>>;
        fn sign(signer: &Signer, method: &str, digest: &[u8]) -> Result<Vec<u8>>;
        fn method(signer: &Signer) -> Result<String>;
        fn profile(signer: &Signer) -> Result<String>;
        fn user_agent(signer: &Signer) -> Result<String>;
        fn using_en_profile(signer: &Signer) -> Result<bool>;
        fn city(signer: &Signer) -> Result<String>;
        fn street_address(signer: &Signer) -> Result<String>;
        fn state_or_province(signer: &Signer) -> Result<String>;
        fn postal_code(signer: &Signer) -> Result<String>;
        fn country_name(signer: &Signer) -> Result<String>;
        fn signer_roles(signer: &Signer) -> Result<Vec<String>>;

        fn set_method(signer: Pin<&mut Signer>, method: &str) -> Result<()>;
        fn set_profile(signer: Pin<&mut Signer>, profile: &str) -> Result<()>;
        fn set_user_agent(signer: Pin<&mut Signer>, user_agent: &str) -> Result<()>;
        fn set_en_profile(signer: Pin<&mut Signer>, enable: bool) -> Result<()>;
        fn set_signature_production_place(
            signer: Pin<&mut Signer>,
            city: &str,
            state_or_province: &str,
            postal_code: &str,
            country_name: &str,
        ) -> Result<()>;
        fn set_signature_production_place_v2(
            signer: Pin<&mut Signer>,
            city: &str,
            street_address: &str,
            state_or_province: &str,
            postal_code: &str,
            country_name: &str,
        ) -> Result<()>;
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

#[cfg(windows)]
impl Debug for ffi::WinSigner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("WinSigner").finish_non_exhaustive()
    }
}
