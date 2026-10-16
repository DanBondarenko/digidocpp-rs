#![expect(deprecated, reason = "strum")]

use cxx::Exception;
use libdigidocpp_sys::exception::ffi;
use std::fmt::{Debug, Display, Formatter};
use strum::FromRepr;
use thiserror::Error as ThisError;

/// Error returned by this crate.
///
/// Most errors come from libdigidocpp. Use [`Error::as_digidoc`] to inspect them, or
/// [`Error::has_code`] to test for a specific failure such as a wrong PIN.
///
/// Any call into libdigidocpp can fail, because the C++ code throws.
///
/// # Examples
///
/// ```no_run
/// use digidocpp::{Container, ExceptionCode, Pkcs11Signer};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let mut container = Container::create("document.asice")?;
/// # let mut signer = Pkcs11Signer::new()?;
/// if let Err(error) = container.sign(&mut signer) {
///     if error.has_code(ExceptionCode::PINIncorrect) {
///         eprintln!("wrong PIN");
///     }
/// }
/// # Ok(())
/// # }
/// ```
#[derive(ThisError, Debug)]
#[non_exhaustive]
pub enum Error {
    /// libdigidocpp threw any exception
    #[error(transparent)]
    Library(LibraryError),
    /// This crate rejected the input before calling libdigidocpp, for example a path that is not
    /// valid UTF-8.
    #[error("internal error: {0}")]
    Internal(String),
    /// The Rust/C++ boundary failed, for example a C++ exception could not be converted to Rust.
    #[error("FFI error: {0}")]
    Ffi(String),
    /// A reader or writer passed to libdigidocpp returned an error.
    #[error("I/O error: {0}")]
    Io(std::io::Error),
}

impl Error {
    fn from_exception_data(data: ffi::ExceptionData) -> Self {
        if data.is_digidoc {
            Self::Library(LibraryError::DigiDoc(DigiDocError {
                raw_code: data.code.repr,
                message: data.message,
                cpp_file: data.file,
                cpp_line: data.line,
                causes: errors_from_dfs_causes(&data.causes),
            }))
        } else if data.is_ffi {
            Self::Ffi(data.message)
        } else {
            Self::Library(LibraryError::Unknown(data.message))
        }
    }

    /// Returns a representation of the upstream [`digidoc::Exception`], if this error is one.
    ///
    /// [`digidoc::Exception`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Exception.html
    #[must_use]
    pub const fn as_digidoc(&self) -> Option<&DigiDocError> {
        match self {
            Self::Library(LibraryError::DigiDoc(error)) => Some(error),
            _ => None,
        }
    }

    /// Returns whether this error, or any of its causes, has `code`.
    ///
    /// `code` corresponds to the upstream [`digidoc::Exception::ExceptionCode`].
    ///
    /// [`digidoc::Exception::ExceptionCode`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Exception.html
    #[must_use]
    pub fn has_code(&self, code: ExceptionCode) -> bool {
        self.as_digidoc().is_some_and(|error| error.has_code(code))
    }
}

/// A representation of an exception thrown by libdigidocpp
#[derive(ThisError, Debug, Clone)]
#[non_exhaustive]
pub enum LibraryError {
    /// A representation of upstream [`digidoc::Exception`] with code and causes.
    ///
    /// [`digidoc::Exception`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Exception.html
    #[error(transparent)]
    DigiDoc(DigiDocError),
    /// Any other C++ exception. Only its message is kept.
    #[error("unknown C++ error: {0}")]
    Unknown(String),
}

/// A representation of [`digidoc::Exception`]: a message and code, the C++ source location that
/// threw it, and the tree of causes that led to it.
///
/// A failed signature validation, for example, is one error whose causes are the individual
/// failed checks.
///
/// [`digidoc::Exception`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Exception.html
#[derive(Debug, Clone)]
pub struct DigiDocError {
    raw_code: i32,
    message: String,
    cpp_file: String,
    cpp_line: i32,
    causes: Vec<Self>,
}

impl DigiDocError {
    /// Returns the message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the message.
    #[must_use]
    pub fn into_message(self) -> String {
        self.message
    }

    /// Returns the exception's [`digidoc::Exception::ExceptionCode`] or `None` if this crate does
    /// not know the value.
    ///
    /// The raw integer representing the code is accessible via [`raw_code`](Self::raw_code).
    ///
    /// [`digidoc::Exception::ExceptionCode`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Exception.html
    #[must_use]
    pub const fn code(&self) -> Option<ExceptionCode> {
        ExceptionCode::from_repr(self.raw_code)
    }

    /// Returns the integer representation of the exception's [`digidoc::Exception::ExceptionCode`].
    ///
    /// [`digidoc::Exception::ExceptionCode`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Exception.html
    #[must_use]
    pub const fn raw_code(&self) -> i32 {
        self.raw_code
    }

    /// Returns the libdigidocpp source file that threw the exception.
    #[must_use]
    pub fn cpp_file(&self) -> &str {
        &self.cpp_file
    }

    /// Returns the libdigidocpp source file that threw the exception.
    #[must_use]
    pub fn into_cpp_file(self) -> String {
        self.cpp_file
    }

    /// Returns the line in [`cpp_file`](Self::cpp_file) that threw the exception.
    #[must_use]
    pub const fn cpp_line(&self) -> i32 {
        self.cpp_line
    }

    /// Returns its direct causes.
    ///
    /// Does not traverse deeper into the causes of those causes.
    #[must_use]
    pub fn into_causes(self) -> Vec<Self> {
        self.causes
    }

    /// Returns its direct causes.
    ///
    /// Does not traverse deeper into the causes of those causes.
    #[must_use]
    pub fn causes(&self) -> &[Self] {
        &self.causes
    }

    /// Iterates over this error and all its causal descendants, depth first.
    pub fn iter_tree(&self) -> impl Iterator<Item = &Self> {
        let mut stack = vec![self];

        std::iter::from_fn(move || {
            let error = stack.pop()?;
            stack.extend(error.causes.iter().rev());
            Some(error)
        })
    }

    /// Returns whether this error, or any causal descendant, has `code`.
    ///
    /// `code` refers to the exception's [`digidoc::Exception::ExceptionCode`].
    ///
    /// [`digidoc::Exception::ExceptionCode`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Exception.html
    #[must_use]
    pub fn has_code(&self, code: ExceptionCode) -> bool {
        self.iter_tree().any(|error| error.code() == Some(code))
    }

    fn from_cause(cause: &ffi::ExceptionCause, subcauses: Vec<Self>) -> Self {
        Self {
            raw_code: cause.code.repr,
            message: cause.message.clone(),
            cpp_file: cause.file.clone(),
            cpp_line: cause.line,
            causes: subcauses,
        }
    }
}

/// Builds errors with nested causes from a flat list of causes.
///
/// The list must be in depth-first order. A cause nests under the nearest earlier cause with a
/// lower `depth`.
fn errors_from_dfs_causes(causes: &[ffi::ExceptionCause]) -> Vec<DigiDocError> {
    let mut errors = Vec::new();
    let mut remaining = causes;
    while let Some((cause, rest)) = remaining.split_first() {
        let descendants_count = rest
            .iter()
            .take_while(|next| next.depth > cause.depth)
            .count();
        let (descendants, siblings) = rest.split_at(descendants_count);
        errors.push(DigiDocError::from_cause(
            cause,
            errors_from_dfs_causes(descendants),
        ));
        remaining = siblings;
    }

    errors
}

impl Display for DigiDocError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let code = self.code().map_or_else(
            || format!("unknown code {}", self.raw_code()),
            |code| format!("{code:?}"),
        );
        let message = &self.message();
        let file = &self.cpp_file();
        let line = self.cpp_line();

        write!(
            f,
            "digidoc error: {code}: {message} (at C++ library source: {file}:{line})"
        )
    }
}

impl std::error::Error for DigiDocError {
    /// Returns the first cause.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.causes
            .first()
            .map(|cause| -> &(dyn std::error::Error + 'static) { cause })
    }
}

/// Error code of a [`DigiDocError`].
///
/// Corresponds to the [`digidoc::Exception::ExceptionCode`] of the underlying [`digidoc::Exception`].
///
/// Codes from `ReferenceDigestWeak` to `MimeTypeWarning` are warnings: validation reports them
/// as causes of a failed [`Signature::validate`](crate::Signature::validate), but the signature
/// itself is sound.
///
/// [`digidoc::Exception::ExceptionCode`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Exception.html
/// [`digidoc::Exception`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Exception.html
#[derive(Clone, Debug, Copy, FromRepr, Eq, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum ExceptionCode {
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
    /// A referenced object uses a weak digest (SHA-1).
    ReferenceDigestWeak = 12,
    /// The signed digest is weak (SHA-1).
    SignatureDigestWeak = 13,
    /// DDOC: a `DataFile` element has no `xmlns` attribute.
    DataFileNameSpaceWarning = 14,
    /// DDOC: an `X509IssuerName` or `X509IssuerSerial` element has no `xmlns` attribute.
    IssuerNameSpaceWarning = 15,
    /// The time-stamp and the OCSP response are more than 15 minutes apart.
    ProducedATLateWarning = 16,
    /// A data file media type is not a valid MIME type.
    MimeTypeWarning = 17,
    /// Unused.
    #[deprecated]
    DDocError = 512,
}

impl From<Exception> for Error {
    fn from(cxx_exception: Exception) -> Self {
        Self::from_exception_data(ffi::ExceptionData::from(cxx_exception))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ffi::ExceptionCode as FfiCode;
    use itertools::Itertools;
    use rstest::rstest;

    struct Expectation {
        message: String,
        causes: Vec<Self>,
    }

    impl Expectation {
        fn leaf(message: impl Into<String>) -> Self {
            Self {
                message: message.into(),
                causes: Vec::new(),
            }
        }

        fn node(message: impl Into<String>, causes: impl Into<Vec<Self>>) -> Self {
            Self {
                message: message.into(),
                causes: causes.into(),
            }
        }

        fn assert(&self, error: &Error) {
            self.assert_digidoc(error.as_digidoc().expect("Expected error of type DigiDoc"));
        }

        fn assert_digidoc(&self, error: &DigiDocError) {
            let code = error.code().expect("Expected code to be identified");
            let expected_message_start = format!("digidoc error: {:?}: {}", code, self.message);
            assert!(
                error.to_string().starts_with(&expected_message_start),
                "Unexpected message \"{error}\". Expected start \"{expected_message_start}\"."
            );
            assert_eq!(self.causes.len(), error.causes().len());

            self.causes
                .iter()
                .zip_eq(error.causes())
                .for_each(|(expected, actual)| expected.assert_digidoc(actual));
        }
    }

    fn mock_exception_data<'a>(
        message: &'a str,
        causes: impl IntoIterator<Item = (&'a str, u32)>,
    ) -> ffi::ExceptionData {
        let causes = causes
            .into_iter()
            .map(|(message, depth)| ffi::ExceptionCause {
                code: ffi::ExceptionCode::General,
                message: String::from(message),
                file: String::new(),
                line: 0,
                depth,
            })
            .collect();

        ffi::ExceptionData {
            is_ffi: false,
            is_digidoc: true,
            is_unknown: false,
            message: String::from(message),
            code: ffi::ExceptionCode::General,
            file: String::new(),
            line: 0,
            causes,
        }
    }

    #[test]
    fn builds_single_top_level_error() {
        let error = Error::from_exception_data(mock_exception_data("a", []));
        let expectation = Expectation::leaf("a");

        expectation.assert(&error);
    }

    #[test]
    fn builds_top_level_error_with_cause() {
        let error = Error::from_exception_data(mock_exception_data("a", [("b", 0)]));
        let expectation = Expectation::node("a", [Expectation::leaf("b")]);

        expectation.assert(&error);
    }

    #[test]
    fn keeps_the_order_of_causes() {
        let error = Error::from_exception_data(mock_exception_data("a", [("b", 0), ("c", 0)]));
        let expectation = Expectation::node("a", [Expectation::leaf("b"), Expectation::leaf("c")]);

        expectation.assert(&error);
    }

    #[test]
    fn nests_a_deeper_entry_under_the_entry_before_it() {
        let error =
            Error::from_exception_data(mock_exception_data("a", [("b", 0), ("c", 1), ("d", 0)]));
        let expectation = Expectation::node(
            "a",
            [
                Expectation::node("b", [Expectation::leaf("c")]),
                Expectation::leaf("d"),
            ],
        );

        expectation.assert(&error);
    }

    #[test]
    fn builds_a_chain_of_causes() {
        let error =
            Error::from_exception_data(mock_exception_data("a", [("b", 0), ("c", 1), ("d", 2)]));
        let expectation = Expectation::node(
            "a",
            [Expectation::node(
                "b",
                [Expectation::node("c", [Expectation::leaf("d")])],
            )],
        );

        expectation.assert(&error);
    }

    #[test]
    fn builds_siblings_at_several_levels() {
        let error = Error::from_exception_data(mock_exception_data(
            "a",
            [("b", 0), ("c", 1), ("d", 1), ("e", 2), ("f", 0)],
        ));
        let expectation = Expectation::node(
            "a",
            [
                Expectation::node(
                    "b",
                    [
                        Expectation::leaf("c"),
                        Expectation::node("d", [Expectation::leaf("e")]),
                    ],
                ),
                Expectation::leaf("f"),
            ],
        );

        expectation.assert(&error);
    }

    #[test]
    fn returns_to_an_outer_level_after_a_deep_chain() {
        let error = Error::from_exception_data(mock_exception_data(
            "a",
            [("b", 0), ("c", 1), ("d", 2), ("e", 1), ("f", 0)],
        ));
        let expectation = Expectation::node(
            "a",
            [
                Expectation::node(
                    "b",
                    [
                        Expectation::node("c", [Expectation::leaf("d")]),
                        Expectation::leaf("e"),
                    ],
                ),
                Expectation::leaf("f"),
            ],
        );

        expectation.assert(&error);
    }

    fn exception_with_codes(
        code: FfiCode,
        causes: impl IntoIterator<Item = (FfiCode, u32)>,
    ) -> Error {
        Error::from_exception_data(ffi::ExceptionData {
            code,
            causes: causes
                .into_iter()
                .map(|(code, depth)| ffi::ExceptionCause {
                    code,
                    depth,
                    message: String::from("cause"),
                    file: String::new(),
                    line: 0,
                })
                .collect(),
            ..mock_exception_data("top", [])
        })
    }

    #[rstest]
    #[case::no_causes(&[], &["a"])]
    #[case::chain(&[("b", 0), ("c", 1), ("d", 2)], &["a", "b", "c", "d"])]
    #[case::siblings(&[("b", 0), ("c", 0)], &["a", "b", "c"])]
    #[case::mixed(
        &[("b", 0), ("c", 1), ("d", 1), ("e", 2), ("f", 0)],
        &["a", "b", "c", "d", "e", "f"]
    )]
    fn iterates_in_expected_order(#[case] causes: &[(&str, u32)], #[case] expected: &[&str]) {
        let error = Error::from_exception_data(mock_exception_data("a", causes.iter().copied()));

        let messages: Vec<_> = error
            .as_digidoc()
            .unwrap()
            .iter_tree()
            .map(DigiDocError::message)
            .collect();

        assert_eq!(messages, expected);
    }

    #[rstest]
    #[case::pos_cause(
        exception_with_codes(FfiCode::General, [(FfiCode::General, 0), (FfiCode::PINIncorrect, 1)]),
        ExceptionCode::PINIncorrect,
        true
    )]
    #[case::neg_cause(
        exception_with_codes(FfiCode::General, [(FfiCode::General, 0), (FfiCode::PINIncorrect, 1)]),
        ExceptionCode::PINLocked,
        false
    )]
    #[case::pos_self(
        exception_with_codes(FfiCode::PINLocked, []),
        ExceptionCode::PINLocked,
        true
    )]
    #[case::neg_self(
        exception_with_codes(FfiCode::PINLocked, []),
        ExceptionCode::PINIncorrect,
        false
    )]
    #[case::not_digidoc(Error::Internal(String::from("a")), ExceptionCode::PINIncorrect, false)]
    fn has_code_searches_error_and_causes(
        #[case] error: Error,
        #[case] code: ExceptionCode,
        #[case] expected: bool,
    ) {
        assert_eq!(error.has_code(code), expected);
    }

    #[test]
    fn has_no_digidoc_error_when_error_is_not_from_digidoc() {
        let error = Error::Internal(String::from("a"));
        assert!(error.as_digidoc().is_none(), "{error:?}");
    }
}
