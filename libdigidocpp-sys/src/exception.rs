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
    struct ExceptionData {
        code: ExceptionCode,
        message: String,
        file: String,
        line: i32,
        is_file_and_line_determined: bool,
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
}

impl ffi::ExceptionData {
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl From<serde_json::Error> for ffi::ExceptionData {
    fn from(error: serde_json::Error) -> Self {
        Self {
            code: ffi::ExceptionCode::General,
            message: error.to_string(),
            file: String::new(),
            line: 0i32,
            is_file_and_line_determined: false,
        }
    }
}

impl From<cxx::Exception> for ffi::ExceptionData {
    fn from(exception: cxx::Exception) -> Self {
        serde_json::from_str(exception.what()).unwrap_or_else(serde_json::Error::into)
    }
}
