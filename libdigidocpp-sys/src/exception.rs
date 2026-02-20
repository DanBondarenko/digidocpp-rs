use std::error::Error;
use std::fmt::Display;

#[cxx::bridge]
pub mod ffi {
    /// See `digidoc::Exception::ExceptionCode` in libdigidocpp
    #[namespace = "digidoc"]
    #[derive(Debug, Clone)]
    #[repr(i32)]
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
        DDocError = 512, //DIGIDOCPP_DEPRECATED
    }

    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/exception_ffi.h");
        type ExceptionCode;
    }

    #[namespace = "digidoc_rs::exception"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/exception_ffi.h");
        fn magic() -> String;
    }
}

#[derive(Debug, Clone)]
pub struct FfiException {
    pub code: ffi::ExceptionCode,
    pub message: String,
    pub file: String,
    pub line: i32,
}

impl FfiException {
    pub fn parse(value: &str) -> Option<Self> {
        let bytes = value.as_bytes();
        let magic = ffi::magic();
        let magic = magic.as_bytes();

        if bytes.len() < 4 + 24 || &bytes[0..4] != magic {
            return None;
        }
        let bytes = &bytes[4..];

        let code = u32::from_str_radix(std::str::from_utf8(&bytes[0..8]).ok()?, 16).ok()? as i32;
        let line = u32::from_str_radix(std::str::from_utf8(&bytes[8..16]).ok()?, 16).ok()? as i32;
        let file_len = u32::from_str_radix(std::str::from_utf8(&bytes[16..24]).ok()?, 16).ok()? as usize;

        let rest = &bytes[24..];
        if rest.len() < file_len {
            return None;
        }
        let (file, message) = rest.split_at(file_len);

        Some(FfiException {
            code: ffi::ExceptionCode { repr: code },
            message: String::from_utf8(message.to_vec()).ok()?,
            file: String::from_utf8(file.to_vec()).ok()?,
            line,
        })
    }
}

impl From<cxx::Exception> for FfiException {
    fn from(exception: cxx::Exception) -> Self {
        Self::parse(exception.what()).unwrap_or_else(|| FfiException {
            code: ffi::ExceptionCode { repr: -1 },
            message: exception.what().to_owned(),
            file: String::new(),
            line: 0,
        })
    }
}

impl Display for FfiException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}:{})", self.message, self.file, self.line)
    }
}

impl Error for FfiException {}

