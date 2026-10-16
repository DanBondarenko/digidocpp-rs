//! Signatures and validation results

#![expect(deprecated, reason = "strum")]

use crate::algorithm::{Algorithm, SignatureProfile};
use crate::error::ExceptionCode;
use crate::stream::call_then_deliver_stream_failure;
use crate::util::{
    ffi_variant_doc, io_variant_doc, library_and_ffi_variants_doc, library_io_and_ffi_variants_doc,
    library_variant_doc, reader_or_writer_panic_doc,
};
use crate::{DerCert, Error, PartialSigner};
use libdigidocpp_sys::signature::ffi;
use std::fmt::{Debug, Display, Formatter};
use std::pin::Pin;
use strum::{AsRefStr, Display as StrumDisplay, EnumString, FromRepr};

/// SiVa validation policy, see the
/// [SiVa documentation](https://open-eid.github.io/SiVa/siva/appendix/validation_policy/).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, AsRefStr, StrumDisplay, EnumString)]
#[non_exhaustive]
pub enum ValidationPolicy {
    /// Accepts signatures regardless of their legal type: qualified or not.
    #[strum(serialize = "POLv1")]
    PolV1,
    /// Accepts qualified signatures and seals only. [`Signature::validate`] uses it.
    #[strum(serialize = "POLv2")]
    PolV2,
}

/// Overall validation result, see [`ValidatorResult::status`].
#[derive(Clone, Debug, Copy, FromRepr, Eq, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum ValidatorStatus {
    /// Valid, and the signature is qualified.
    Valid = 0,
    /// Valid, with warnings. See [`ValidatorResult::warnings`].
    Warning = 1,
    /// Valid, but the signature is not qualified.
    ///
    /// The signer certificate is not a qualified certificate, or its key is not on a qualified
    /// device.
    NonQSCD = 2,
    /// Unused.
    #[deprecated]
    Test = 3, //DIGIDOCPP_DEPRECATED
    /// Invalid.
    Invalid = 4,
    /// Validity could not be determined, for example because certificates are missing.
    Unknown = 5,
}

/// A signature inside a [`Container`](crate::Container).
///
/// Getters that return text return an empty string when the signature does not have the value.
/// For example, a signature without a time-stamp has an empty
/// [`time_stamp_time`](Self::time_stamp_time).
///
/// Wraps upstream [`digidoc::Signature`].
///
/// # Examples
///
/// ```no_run
/// use digidocpp::Container;
///
/// # fn main() -> Result<(), digidocpp::Error> {
/// let container = Container::open("document.asice")?;
/// for signature in container.signatures()? {
///     let result = signature.validator_result()?;
///     println!("{}: {:?}", signature.signed_by()?, result.status());
/// }
/// # Ok(())
/// # }
/// ```
///
/// [`digidoc::Signature`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signature.html
#[derive(Clone, Copy)]
pub struct Signature<'a>(pub(crate) &'a ffi::Signature);

/// A signature inside a [`Container`](crate::Container), for external signing and extension.
///
/// Use [`as_signature`](Self::as_signature) to read it. See [`Container::prepare_signature`](crate::Container::prepare_signature).
///
/// Wraps upstream [`digidoc::Signature`].
///
/// [`digidoc::Signature`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signature.html
pub struct SignatureMut<'a>(pub(crate) Pin<&'a mut ffi::Signature>);

/// An archive time-stamp of a signature.
///
/// Corresponds to upstream [`digidoc::TSAInfo`].
///
/// [`digidoc::TSAInfo`]: https://open-eid.github.io/libdigidocpp/structdigidoc_1_1TSAInfo.html
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TSAInfo {
    cert: Option<DerCert>,
    time: String,
}

impl TSAInfo {
    pub(crate) fn from_ffi(info: ffi::TSAInfo) -> Self {
        Self {
            cert: DerCert::from_ffi(info.cert),
            time: info.time,
        }
    }

    /// Returns the certificate of the time-stamp authority.
    #[must_use]
    pub const fn cert(&self) -> Option<&DerCert> {
        self.cert.as_ref()
    }

    /// Returns the time-stamp time.
    #[must_use]
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Returns the certificate.
    #[must_use]
    pub fn into_cert(self) -> Option<DerCert> {
        self.cert
    }

    /// Returns the time.
    #[must_use]
    pub fn into_time(self) -> String {
        self.time
    }
}

/// Outcome of [`Signature::validator_result`]: a status plus the reasons behind it.
///
/// Corresponds to upstream [`digidoc::Signature::Validator`].
///
/// [`digidoc::Signature::Validator`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Signature_1_1Validator.html
#[derive(Debug, Clone)]
pub struct ValidatorResult(ffi::ValidatorResult);

impl ValidatorResult {
    /// Returns the status, or `None` if libdigidocpp reported a value this crate does not know.
    #[must_use]
    pub const fn status(&self) -> Option<ValidatorStatus> {
        ValidatorStatus::from_repr(self.0.status.repr)
    }

    /// Returns the status as libdigidocpp reports it.
    #[must_use]
    pub const fn raw_status(&self) -> i32 {
        self.0.status.repr
    }

    /// Returns the message of every failed check or warning, one per line.
    #[must_use]
    pub fn diagnostics(&self) -> &str {
        &self.0.diagnostics
    }

    /// Returns [`diagnostics`](Self::diagnostics).
    #[must_use]
    pub fn into_diagnostics(self) -> String {
        self.0.diagnostics
    }

    /// Returns the codes of the warnings.
    ///
    /// Each code has a decoded value (`None` if unknown) and the raw number.
    #[must_use]
    pub fn warnings(&self) -> Vec<(Option<ExceptionCode>, i32)> {
        self.0
            .warnings
            .iter()
            .map(|code| (ExceptionCode::from_repr(code.repr), code.repr))
            .collect()
    }
}

impl Signature<'_> {
    /// Returns the signature ID within the container, such as `S0`.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn id(&self) -> Result<String, Error> {
        Ok(ffi::id(self.0)?)
    }

    /// Returns the signing time the signer claimed, from the signer's clock.
    ///
    /// Nothing proves it.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn claimed_signing_time(&self) -> Result<String, Error> {
        Ok(ffi::claimed_signing_time(self.0)?)
    }

    /// Returns the earliest time the signature is proven to exist.
    ///
    /// That is the time-stamp time for `time-stamp` profiles, the OCSP `producedAt` for
    /// `time-mark`, and the claimed time otherwise.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn trusted_signing_time(&self) -> Result<String, Error> {
        Ok(ffi::trusted_signing_time(self.0)?)
    }

    /// Returns the certificate that signed.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn signing_certificate(&self) -> Result<Option<DerCert>, Error> {
        Ok(DerCert::from_ffi(ffi::signing_certificate(self.0)?))
    }

    /// Returns the algorithm used to sign.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn signature_method(&self) -> Result<Algorithm, Error> {
        Ok(Algorithm::from(ffi::signature_method(self.0)?.as_str()))
    }

    /// Validates the signature against [`ValidationPolicy::PolV2`].
    ///
    /// Use [`validator_result`](Self::validator_result) to tell warnings from errors.
    ///
    /// # Errors
    /// Returns [`Error`] when a check fails or the signature has warnings, such as a weak digest.
    /// The failed checks are the causes (see
    /// [`DigiDocError::causes`](crate::error::DigiDocError::causes)).
    #[doc = io_variant_doc!()]
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
    pub fn validate(&self) -> Result<(), Error> {
        call_then_deliver_stream_failure(|| ffi::validate(self.0))
    }

    /// Validates the signature against `policy`.
    ///
    /// [`validate`](Self::validate) uses [`ValidationPolicy::PolV2`].
    ///
    /// # Errors
    /// See [`validate`](Self::validate).
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
    pub fn validate_with_policy(&self, policy: ValidationPolicy) -> Result<(), Error> {
        call_then_deliver_stream_failure(|| ffi::validate_with_policy(self.0, policy.as_ref()))
    }

    /// Returns the digest to sign.
    ///
    /// The signature comes from
    /// [`Container::prepare_signature`](crate::Container::prepare_signature). The digest is of the
    /// canonicalized `SignedInfo`, computed with the digest of
    /// [`signature_method`](Self::signature_method).
    ///
    /// # Errors
    #[doc = library_io_and_ffi_variants_doc!()]
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
    pub fn data_to_sign(&self) -> Result<Vec<u8>, Error> {
        call_then_deliver_stream_failure(|| ffi::data_to_sign(self.0))
    }

    /// Returns the signature policy OID.
    ///
    /// Empty when the signature has no explicit policy.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn policy(&self) -> Result<String, Error> {
        Ok(ffi::policy(self.0)?)
    }

    /// Returns the signature policy URI.
    ///
    /// Empty when the signature has no explicit policy.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn sp_uri(&self) -> Result<String, Error> {
        Ok(ffi::sp_uri(self.0)?)
    }

    /// Returns the signature profile.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn profile(&self) -> Result<SignatureProfile, Error> {
        Ok(SignatureProfile::from(ffi::profile(self.0)?.as_str()))
    }

    /// Returns the city of the signature production place.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn city(&self) -> Result<String, Error> {
        Ok(ffi::city(self.0)?)
    }

    /// Returns the state or province of the signature production place.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn state_or_province(&self) -> Result<String, Error> {
        Ok(ffi::state_or_province(self.0)?)
    }

    /// Returns the postal code of the signature production place.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn postal_code(&self) -> Result<String, Error> {
        Ok(ffi::postal_code(self.0)?)
    }

    /// Returns the country of the signature production place.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn country_name(&self) -> Result<String, Error> {
        Ok(ffi::country_name(self.0)?)
    }

    /// Returns the street address of the signature production place.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn street_address(&self) -> Result<String, Error> {
        Ok(ffi::street_address(self.0)?)
    }

    /// Returns the claimed signer roles.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn signer_roles(&self) -> Result<Vec<String>, Error> {
        Ok(ffi::signer_roles(self.0)?)
    }

    /// Returns the `producedAt` time of the OCSP response.
    ///
    /// Empty without an OCSP response.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn ocsp_produced_at(&self) -> Result<String, Error> {
        Ok(ffi::ocsp_produced_at(self.0)?)
    }

    /// Returns the certificate of the OCSP responder.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn ocsp_certificate(&self) -> Result<Option<DerCert>, Error> {
        Ok(DerCert::from_ffi(ffi::ocsp_certificate(self.0)?))
    }

    /// Returns the certificate of the authority that issued the signature time-stamp.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn time_stamp_certificate(&self) -> Result<Option<DerCert>, Error> {
        Ok(DerCert::from_ffi(ffi::time_stamp_certificate(self.0)?))
    }

    /// Returns the time of the signature time-stamp.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn time_stamp_time(&self) -> Result<String, Error> {
        Ok(ffi::time_stamp_time(self.0)?)
    }

    /// Returns the archive time-stamps, oldest first.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn archive_time_stamps(&self) -> Result<Vec<TSAInfo>, Error> {
        Ok(ffi::archive_time_stamps(self.0)?
            .into_iter()
            .map(TSAInfo::from_ffi)
            .collect())
    }

    /// Returns the common name of the signing certificate.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn signed_by(&self) -> Result<String, Error> {
        Ok(ffi::signed_by(self.0)?)
    }

    /// Returns the value that the time-stamp or time-mark covers.
    ///
    /// It is the digest in the time-stamp, or the OCSP nonce for `time-mark` profiles.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn message_imprint(&self) -> Result<Vec<u8>, Error> {
        Ok(ffi::message_imprint(self.0)?)
    }

    /// Validates the signature and classifies the outcome.
    ///
    /// Runs [`validate`](Self::validate). If the result is [`ValidatorStatus::Unknown`], it
    /// retries with [`ValidationPolicy::PolV1`] and reports [`ValidatorStatus::NonQSCD`] when
    /// that passes.
    ///
    /// # Errors
    #[doc = library_io_and_ffi_variants_doc!()]
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
    pub fn validator_result(&self) -> Result<ValidatorResult, Error> {
        call_then_deliver_stream_failure(|| ffi::validator(self.0)).map(ValidatorResult)
    }
}

impl SignatureMut<'_> {
    #[must_use]
    /// Reborrows the signature for reading.
    pub fn as_signature(&self) -> Signature<'_> {
        Signature(self.0.as_ref().get_ref())
    }

    /// Sets the signature value, produced by signing [`Signature::data_to_sign`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_signature_value(&mut self, value: &[u8]) -> Result<(), Error> {
        Ok(ffi::set_signature_value(self.0.as_mut(), value)?)
    }

    /// Adds what the profile of `signer` needs after signing.
    ///
    /// Profiles with `time-stamp` add a time-stamp and an OCSP response.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when the time-stamp or OCSP request fails.
    #[doc = concat!(io_variant_doc!(), ffi_variant_doc!())]
    ///
    /// # Panics
    #[doc = reader_or_writer_panic_doc!()]
    pub fn extend_signature_profile<S>(&mut self, signer: &mut S) -> Result<(), Error>
    where
        S: PartialSigner + ?Sized,
    {
        let signature = self.0.as_mut();
        let signer = signer.ffi_signer_mut();

        call_then_deliver_stream_failure(move || ffi::extend_signature_profile(signature, signer))
    }
}

impl Display for Signature<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let id = self.id().unwrap_or_else(|_| String::from("<unknown>"));
        let signed_by = self
            .signed_by()
            .unwrap_or_else(|_| String::from("<unknown>"));
        let method = self
            .signature_method()
            .map_or_else(|_| String::from("<unknown>"), |method| method.to_string());
        let date = self
            .trusted_signing_time()
            .unwrap_or_else(|_| String::from("<unknown>"));
        let profile = self
            .profile()
            .map_or_else(|_| String::from("<unknown>"), |profile| profile.to_string());
        write!(
            f,
            "Signature {id} signed by {signed_by} with method {method} and profile {profile} at {date}"
        )
    }
}

impl Debug for Signature<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signature")
            .field("id()", &self.id())
            .field("sp_uri()", &self.sp_uri())
            .field("policy()", &self.policy())
            .field("profile()", &self.profile())
            .field("signed_by()", &self.signed_by())
            .field("signer_roles()", &self.signer_roles())
            .field("signature_method()", &self.signature_method())
            .field("trusted_signing_time()", &self.trusted_signing_time())
            .finish_non_exhaustive()
    }
}

impl Display for SignatureMut<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.as_signature(), f)
    }
}

impl Debug for SignatureMut<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SignatureMut")
            .field(&self.as_signature())
            .finish()
    }
}
