use cxx::Exception;
use libdigidocpp_sys::exception::ffi;
use libdigidocpp_sys::exception::ffi::ExceptionData;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug, Copy, IntoPrimitive, TryFromPrimitive, Eq, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum ExceptionCode {
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
    DDocError = 512,
}

impl From<ffi::ExceptionCode> for ExceptionCode {
    fn from(value: ffi::ExceptionCode) -> Self {
        Self::try_from(value.repr).unwrap_or({
            // TODO: Log?
            Self::General
        })
    }
}

#[derive(Debug, Clone)]
pub struct Error(ExceptionData);

impl Error {
    #[must_use]
    pub fn message(&self) -> &str {
        &self.0.message
    }

    #[must_use]
    pub fn code(&self) -> ExceptionCode {
        self.0.code.into()
    }

    #[must_use]
    pub fn cpp_file_and_line(&self) -> Option<(&str, i32)> {
        if self.0.is_file_and_line_determined {
            Some((&self.0.file, self.0.line))
        } else {
            None
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let code = self.0.code;
        let message = &self.0.message;

        if let Some((file, line)) = self.cpp_file_and_line() {
            write!(
                f,
                "Error {code:?}: \"{message}\" (at C++ library source: {file}:{line})"
            )
        } else {
            write!(f, "Error {code:?}: \"{message}\"")
        }
    }
}

impl std::error::Error for Error {}

impl From<ExceptionData> for Error {
    fn from(value: ExceptionData) -> Self {
        Self(value)
    }
}

impl From<Exception> for Error {
    fn from(value: Exception) -> Self {
        Self(ExceptionData::from(value))
    }
}
