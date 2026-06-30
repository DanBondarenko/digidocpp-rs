#[cxx::bridge]
#[allow(clippy::exhaustive_structs)]
#[allow(clippy::exhaustive_enums)]
pub mod ffi {
    /// See `digidoc::Signature::Validator::Status` in libdigidocpp
    #[namespace = "digidoc"]
    #[derive(Debug, Clone)]
    #[repr(i32)]
    #[allow(deprecated)]
    enum Status {
        Valid,
        Warning,
        NonQSCD,
        #[deprecated]
        Test, //DIGIDOCPP_DEPRECATED
        Invalid,
        Unknown,
    }

    #[namespace = "digidoc_rs::signature"]
    #[derive(Debug, Clone)]
    struct TSAInfo {
        cert: Vec<u8>,
        time: String,
    }

    #[namespace = "digidoc_rs::signature"]
    #[derive(Debug, Clone)]
    struct ValidatorResult {
        status: Status,
        diagnostics: String,
        warnings: Vec<ExceptionCode>,
    }

    impl Vec<ExceptionCode> {}

    #[namespace = "digidoc"]
    #[allow(missing_debug_implementations)]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/signature_ffi.h");
        include!("digidocpp/Signature.h");

        type Signature;
        type Status;
        type ExceptionCode = crate::exception::ffi::ExceptionCode;
    }

    #[namespace = "digidoc_rs::signature"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/signature_ffi.h");
        include!("libdigidocpp-sys/include/exception_ffi.h");

        fn pol_v1() -> Result<String>;
        fn pol_v2() -> Result<String>;

        fn id(signature: &Signature) -> Result<String>;
        fn claimed_signing_time(signature: &Signature) -> Result<String>;
        fn trusted_signing_time(signature: &Signature) -> Result<String>;
        fn signing_certificate(signature: &Signature) -> Result<Vec<u8>>;
        fn signature_method(signature: &Signature) -> Result<String>;

        fn validate(signature: &Signature) -> Result<()>;
        fn validate_with_policy(signature: &Signature, policy: &str) -> Result<()>;
        fn data_to_sign(signature: &Signature) -> Result<Vec<u8>>;

        fn policy(signature: &Signature) -> Result<String>;
        fn sp_uri(signature: &Signature) -> Result<String>;
        fn profile(signature: &Signature) -> Result<String>;
        fn city(signature: &Signature) -> Result<String>;
        fn state_or_province(signature: &Signature) -> Result<String>;
        fn postal_code(signature: &Signature) -> Result<String>;
        fn country_name(signature: &Signature) -> Result<String>;
        fn street_address(signature: &Signature) -> Result<String>;

        fn signer_roles(signature: &Signature) -> Result<Vec<String>>;
        fn ocsp_produced_at(signature: &Signature) -> Result<String>;
        fn ocsp_certificate(signature: &Signature) -> Result<Vec<u8>>;
        fn time_stamp_certificate(signature: &Signature) -> Result<Vec<u8>>;
        fn time_stamp_time(signature: &Signature) -> Result<String>;
        fn archive_time_stamps(signature: &Signature) -> Result<Vec<TSAInfo>>;

        fn signed_by(signature: &Signature) -> Result<String>;
        fn message_imprint(signature: &Signature) -> Result<Vec<u8>>;

        fn validator(signature: &Signature) -> Result<ValidatorResult>;
    }
}
