//! Signatures: upstream [`digidoc::Signature`], bound as [`ffi::Signature`], and
//! [`digidoc::Signature::Validator`].
//!
//! [`digidoc::Signature`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signature.html
//! [`digidoc::Signature::Validator`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signature_1_1Validator.html

use std::fmt::{self, Debug, Formatter};

/// Bridge for signatures.
#[cxx::bridge]
#[allow(clippy::missing_errors_doc, reason = "cxx")]
#[allow(clippy::exhaustive_structs, reason = "cxx")]
#[allow(clippy::exhaustive_enums, reason = "cxx")]
pub mod ffi {
    /// Upstream [`digidoc::Signature::Validator::Status`].
    ///
    /// [`digidoc::Signature::Validator::Status`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signature_1_1Validator.html
    #[namespace = "digidoc"]
    #[derive(Debug, Clone)]
    #[repr(i32)]
    #[allow(deprecated)]
    enum Status {
        /// Valid, and the signature is qualified.
        Valid,
        /// Valid, with warnings. See [`ValidatorResult::warnings`].
        Warning,
        /// Valid, but the signature is not qualified.
        ///
        /// The signer certificate is not a qualified certificate, or its key is not on a qualified
        /// device.
        NonQSCD,
        /// Unused since upstream 3.14.7.
        #[deprecated]
        Test, //DIGIDOCPP_DEPRECATED
        /// Invalid.
        Invalid,
        /// Validity could not be determined.
        Unknown,
    }

    /// Upstream [`digidoc::TSAInfo`].
    ///
    /// [`digidoc::TSAInfo`]: https://open-eid.github.io/libdigidocpp/structdigidoc_1_1TSAInfo.html
    #[namespace = "digidoc_rs::signature"]
    #[derive(Debug, Clone)]
    struct TSAInfo {
        /// DER certificate of the time-stamp authority. Empty if absent.
        cert: Vec<u8>,
        /// Time-stamp time.
        time: String,
    }

    /// Outcome of an upstream [`digidoc::Signature::Validator`].
    ///
    /// [`digidoc::Signature::Validator`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signature_1_1Validator.html
    #[namespace = "digidoc_rs::signature"]
    #[derive(Debug, Clone)]
    struct ValidatorResult {
        /// Upstream `Validator::status`.
        status: Status,
        /// Upstream `Validator::diagnostics`.
        diagnostics: String,
        /// Upstream `Validator::warnings`.
        warnings: Vec<ExceptionCode>,
    }

    impl Vec<ExceptionCode> {}

    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/signature_ffi.h");
        include!("digidocpp/Signature.h");

        /// Upstream [`digidoc::Signature`], owned by its container.
        ///
        /// [`digidoc::Signature`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signature.html
        type Signature;

        /// Upstream [`digidoc::Signer`].
        ///
        /// [`digidoc::Signer`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signer.html
        type Signer = crate::signer::ffi::Signer;

        /// The C++ side of [`Status`].
        type Status;

        /// The C++ side of [`ExceptionCode`](crate::exception::ffi::ExceptionCode).
        type ExceptionCode = crate::exception::ffi::ExceptionCode;
    }

    #[namespace = "digidoc_rs::signature"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/signature_ffi.h");
        include!("libdigidocpp-sys/include/exception_ffi.h");

        /// Calls upstream `Signature::id`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn id(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::claimedSigningTime`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn claimed_signing_time(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::trustedSigningTime`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn trusted_signing_time(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::signingCertificate`.
        ///
        /// Returns DER, or empty if absent.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn signing_certificate(signature: &Signature) -> Result<Vec<u8>>;

        /// Calls upstream `Signature::signatureMethod`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn signature_method(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::validate`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when the signature is
        /// invalid or has warnings.
        fn validate(signature: &Signature) -> Result<()>;

        /// Calls upstream `Signature::validate`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when the signature is
        /// invalid or has warnings.
        fn validate_with_policy(signature: &Signature, policy: &str) -> Result<()>;

        /// Calls upstream `Signature::dataToSign`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn data_to_sign(signature: &Signature) -> Result<Vec<u8>>;

        /// Calls upstream `Signature::setSignatureValue`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_signature_value(signature: Pin<&mut Signature>, value: &[u8]) -> Result<()>;

        /// Calls upstream `Signature::extendSignatureProfile`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn extend_signature_profile(
            signature: Pin<&mut Signature>,
            signer: Pin<&mut Signer>,
        ) -> Result<()>;

        /// Calls upstream `Signature::policy`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn policy(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::SPUri`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn sp_uri(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::profile`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn profile(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::city`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn city(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::stateOrProvince`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn state_or_province(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::postalCode`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn postal_code(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::countryName`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn country_name(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::streetAddress`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn street_address(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::signerRoles`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn signer_roles(signature: &Signature) -> Result<Vec<String>>;

        /// Calls upstream `Signature::OCSPProducedAt`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn ocsp_produced_at(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::OCSPCertificate`.
        ///
        /// Returns DER, or empty if absent.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn ocsp_certificate(signature: &Signature) -> Result<Vec<u8>>;

        /// Calls upstream `Signature::TimeStampCertificate`.
        ///
        /// Returns DER, or empty if absent.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn time_stamp_certificate(signature: &Signature) -> Result<Vec<u8>>;

        /// Calls upstream `Signature::TimeStampTime`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn time_stamp_time(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::ArchiveTimeStamps`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn archive_time_stamps(signature: &Signature) -> Result<Vec<TSAInfo>>;

        /// Calls upstream `Signature::signedBy`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn signed_by(signature: &Signature) -> Result<String>;

        /// Calls upstream `Signature::messageImprint`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn message_imprint(signature: &Signature) -> Result<Vec<u8>>;

        /// Runs an upstream `Signature::Validator` on the signature and collects its outcome.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn validator(signature: &Signature) -> Result<ValidatorResult>;
    }
}

impl Debug for ffi::Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Signature").finish_non_exhaustive()
    }
}
