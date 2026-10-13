//! Transport of C++ exceptions across the boundary.
//!
//! Every C++ exception reaches Rust as a `cxx::Exception`. Its message is an
//! [`ExceptionData`](ffi::ExceptionData) serialized as JSON.

/// Bridge for exception data.
#[cxx::bridge]
#[allow(clippy::exhaustive_structs, reason = "cxx")]
#[allow(clippy::exhaustive_enums, reason = "cxx")]
pub mod ffi {
    /// Upstream `ExceptionCode` of [`digidoc::Exception`].
    ///
    /// [`digidoc::Exception`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Exception.html
    #[namespace = "digidoc"]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[repr(i32)]
    #[allow(deprecated)]
    enum ExceptionCode {
        /// No specific code.
        General = 0,
        /// A network connection failed.
        NetworkError = 20,
        /// A host name did not resolve.
        HostNotFound = 101,
        /// A URL is invalid.
        InvalidUrl = 102,

        /// The issuer certificate of the signing certificate is missing.
        CertificateIssuerMissing = 10,
        /// The OCSP response says the certificate is revoked.
        CertificateRevoked = 5,
        /// The OCSP response says the certificate status is unknown.
        CertificateUnknown = 6,
        /// The OCSP `producedAt` time is before the time-stamp time.
        OCSPBeforeTimeStamp = 19,
        /// There is no OCSP responder URL.
        OCSPResponderMissing = 8,
        /// The OCSP responder certificate is missing.
        OCSPCertMissing = 9,
        /// The OCSP response is outside its valid time window.
        OCSPTimeSlot = 7,
        /// The OCSP responder requires a signed request.
        OCSPRequestUnauthorized = 11,
        /// The time-stamp service answered Forbidden.
        TSForbidden = 21,
        /// The time-stamp service request limit is reached.
        TSTooManyRequests = 18,

        /// The user cancelled the PIN prompt.
        PINCanceled = 2,
        /// PIN verification failed for another reason.
        PINFailed = 4,
        /// The PIN is wrong.
        PINIncorrect = 1,
        /// The PIN is locked.
        PINLocked = 3,

        /// Warning: a referenced object uses a weak digest.
        ReferenceDigestWeak = 12,
        /// Warning: the signed digest is weak.
        SignatureDigestWeak = 13,
        /// Warning, DDOC: a `DataFile` element has no `xmlns` attribute.
        DataFileNameSpaceWarning = 14,
        /// Warning, DDOC: an issuer element has no `xmlns` attribute.
        IssuerNameSpaceWarning = 15,
        /// Warning: time-stamp and OCSP response are more than 15 minutes apart.
        ProducedATLateWarning = 16,
        /// Warning: a media type is not a valid MIME type.
        MimeTypeWarning = 17,

        /// Unused.
        #[deprecated]
        DDocError = 512,
    }

    /// One entry in the cause tree of an exception.
    #[namespace = "digidoc_rs::exception"]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct ExceptionCause {
        /// The cause's code.
        code: ExceptionCode,
        /// The cause's message.
        message: String,
        /// C++ source file that threw the cause.
        file: String,
        /// Line in `file`.
        line: i32,
        /// Nesting level: 0 for a direct cause of the exception, 1 for its causes, and so on.
        depth: u32,
    }

    /// A C++ exception, as sent across the boundary.
    ///
    /// Exactly one of `is_ffi`, `is_digidoc` and `is_unknown` is true.
    #[namespace = "digidoc_rs::exception"]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct ExceptionData {
        // cxx doesn't support enums as sum types unfortunately, we use booleans to determine which
        // fields are valid
        /// The C++ bridge failed, for example it could not serialize the exception.
        is_ffi: bool,
        /// An upstream `digidoc::Exception`.
        is_digidoc: bool,
        /// Any other C++ exception.
        is_unknown: bool,
        /// The exception message.
        message: String,
        /// Only valid if `is_digidoc` is true.
        code: ExceptionCode,
        /// Only valid if `is_digidoc` is true.
        file: String,
        /// Only valid if `is_digidoc` is true.
        line: i32,
        /// The causes of an exception form a flat list in depth-first order.
        /// Only valid if `is_digidoc` is true.
        causes: Vec<ExceptionCause>,
    }

    #[namespace = "digidoc_rs::exception"]
    extern "Rust" {
        /// Serializes the data for the C++ side to throw.
        fn to_json(self: &ExceptionData) -> Result<String>;
    }

    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/exception_ffi.h");

        /// The C++ side of [`ExceptionCode`].
        type ExceptionCode;
    }

    #[cfg(feature = "_test_util")]
    #[namespace = "digidoc_rs::exception"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/test_util.h");

        /// Throws an upstream `digidoc::Exception`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn throw_digidoc_exception(message: &[u8]) -> Result<()>;

        /// Throws a `std::exception`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn throw_std_exception(message: &[u8]) -> Result<()>;

        /// Throws a value that is not a `std::exception`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn throw_non_std_exception() -> Result<()>;

        /// Throws an upstream `digidoc::Exception` with a tree of causes.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn throw_digidoc_exception_with_causes() -> Result<()>;

        /// Throws an upstream `digidoc::Exception` with an unlisted code.
        ///
        /// [`ExceptionCode`] does not list the code.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn throw_digidoc_exception_with_invalid_code() -> Result<()>;
    }
}

impl ffi::ExceptionData {
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// Represents a JSON serialization or parse error at the FFI boundary.
impl From<serde_json::Error> for ffi::ExceptionData {
    fn from(error: serde_json::Error) -> Self {
        Self {
            is_ffi: true,
            is_digidoc: false,
            is_unknown: false,
            message: error.to_string(),

            // Rest of the field values are stubs and meaningless
            code: ffi::ExceptionCode::General,
            file: String::new(),
            line: 0i32,
            causes: Vec::new(),
        }
    }
}

/// Parses the exception message as JSON.
///
/// A message that is not valid JSON becomes an `is_ffi` value.
impl From<cxx::Exception> for ffi::ExceptionData {
    fn from(exception: cxx::Exception) -> Self {
        serde_json::from_str(exception.what()).unwrap_or_else(serde_json::Error::into)
    }
}
