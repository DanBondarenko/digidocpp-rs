#[cxx::bridge]
#[allow(clippy::exhaustive_structs)]
#[allow(clippy::exhaustive_enums)]
pub mod ffi {
    /// See `digidoc::Exception::ExceptionCode` in libdigidocpp
    #[namespace = "digidoc"]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[repr(i32)]
    #[allow(deprecated)]
    enum ExceptionCode {
        General = 0,
        NetworkError = 20,
        HostNotFound = 101,
        InvalidUrl = 102,
        //Verification errors
        CertificateIssuerMissing = 10,
        CertificateRevoked = 5,
        CertificateUnknown = 6,
        OCSPBeforeTimeStamp = 19,
        OCSPResponderMissing = 8,
        OCSPCertMissing = 9,
        OCSPTimeSlot = 7,
        OCSPRequestUnauthorized = 11,
        TSForbidden = 21,
        TSTooManyRequests = 18,
        //Pin exceptions
        PINCanceled = 2,
        PINFailed = 4,
        PINIncorrect = 1,
        PINLocked = 3,
        //Warnings
        ReferenceDigestWeak = 12,
        SignatureDigestWeak = 13,
        DataFileNameSpaceWarning = 14,
        IssuerNameSpaceWarning = 15,
        ProducedATLateWarning = 16,
        MimeTypeWarning = 17,
        //DDoc error codes
        #[deprecated]
        DDocError = 512, //DIGIDOCPP_DEPRECATED
    }

    #[namespace = "digidoc_rs::exception"]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct ExceptionCause {
        code: ExceptionCode,
        message: String,
        file: String,
        line: i32,
        /// 0 for a direct cause growing by 1 for each level of nesting below it
        depth: u32,
    }

    #[namespace = "digidoc_rs::exception"]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct ExceptionData {
        // cxx doesn't support enums as ASTs unfortunately, we use booleans to determine which
        // fields are valid
        is_ffi: bool,
        is_digidoc: bool,
        is_unknown: bool,
        message: String,
        /// Only valid if `is_digidoc` is true
        code: ExceptionCode,
        /// Only valid if `is_digidoc` is true
        file: String,
        /// Only valid if `is_digidoc` is true
        line: i32,
        /// The causes of an exception form a flat list in depth-first order.
        /// Only valid if `is_digidoc` is true.
        causes: Vec<ExceptionCause>,
    }

    #[namespace = "digidoc_rs::exception"]
    extern "Rust" {
        fn to_json(self: &ExceptionData) -> Result<String>;
    }

    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/exception_ffi.h");
        type ExceptionCode;
    }

    #[cfg(feature = "test_util")]
    #[namespace = "digidoc_rs::exception"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/test_util.h");

        fn throw_digidoc_exception(message: &[u8]) -> Result<()>;
        fn throw_std_exception(message: &[u8]) -> Result<()>;
        fn throw_non_std_exception() -> Result<()>;
        fn throw_digidoc_exception_with_causes() -> Result<()>;
        fn throw_digidoc_exception_with_invalid_code() -> Result<()>;
    }
}

impl ffi::ExceptionData {
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl From<serde_json::Error> for ffi::ExceptionData {
    /// Represents a serialization error at the FFI boundary
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

impl From<cxx::Exception> for ffi::ExceptionData {
    fn from(exception: cxx::Exception) -> Self {
        serde_json::from_str(exception.what()).unwrap_or_else(serde_json::Error::into)
    }
}
