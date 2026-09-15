use crate::stream::call_then_deliver_stream_failure;
use crate::{DerCert, Error, ExceptionCode, Signer};
use libdigidocpp_sys::signature::ffi;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::{Debug, Display, Formatter};
use std::pin::Pin;

#[derive(Clone, Debug, Copy, IntoPrimitive, TryFromPrimitive, Eq, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum ValidatorStatus {
    Valid = 0,
    Warning = 1,
    NonQSCD = 2,
    #[deprecated]
    Test = 3, //DIGIDOCPP_DEPRECATED
    Invalid = 4,
    Unknown = 5,
}

impl From<ffi::Status> for ValidatorStatus {
    fn from(value: ffi::Status) -> Self {
        Self::try_from(value.repr).unwrap_or({
            // TODO: Log?
            Self::Unknown
        })
    }
}

#[derive(Clone, Copy)]
pub struct Signature<'a>(pub(crate) &'a ffi::Signature);

pub struct SignatureMut<'a>(pub(crate) Pin<&'a mut ffi::Signature>);

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

    #[must_use]
    pub const fn cert(&self) -> Option<&DerCert> {
        self.cert.as_ref()
    }

    #[must_use]
    pub fn time(&self) -> &str {
        &self.time
    }

    #[must_use]
    pub fn into_cert(self) -> Option<DerCert> {
        self.cert
    }

    #[must_use]
    pub fn into_time(self) -> String {
        self.time
    }
}

#[derive(Debug, Clone)]
pub struct ValidatorResult(ffi::ValidatorResult);

impl ValidatorResult {
    #[must_use]
    pub fn status(&self) -> ValidatorStatus {
        self.0.status.into()
    }

    #[must_use]
    pub fn diagnostics(&self) -> &str {
        &self.0.diagnostics
    }

    #[must_use]
    pub fn into_diagnostics(self) -> String {
        self.0.diagnostics
    }

    #[must_use]
    pub fn warnings(&self) -> Vec<(Option<ExceptionCode>, i32)> {
        self.0
            .warnings
            .iter()
            .map(|code| (code.repr.try_into().ok(), code.repr))
            .collect()
    }
}

impl Signature<'_> {
    pub fn pol_v1() -> Result<String, Error> {
        Ok(ffi::pol_v1()?)
    }

    pub fn pol_v2() -> Result<String, Error> {
        Ok(ffi::pol_v2()?)
    }

    pub fn id(&self) -> Result<String, Error> {
        Ok(ffi::id(self.0)?)
    }

    pub fn claimed_signing_time(&self) -> Result<String, Error> {
        Ok(ffi::claimed_signing_time(self.0)?)
    }

    pub fn trusted_signing_time(&self) -> Result<String, Error> {
        Ok(ffi::trusted_signing_time(self.0)?)
    }

    pub fn signing_certificate(&self) -> Result<Option<DerCert>, Error> {
        Ok(DerCert::from_ffi(ffi::signing_certificate(self.0)?))
    }

    pub fn signature_method(&self) -> Result<String, Error> {
        Ok(ffi::signature_method(self.0)?)
    }

    pub fn validate(&self) -> Result<(), Error> {
        call_then_deliver_stream_failure(|| ffi::validate(self.0))
    }

    pub fn validate_with_policy(&self, policy: &str) -> Result<(), Error> {
        call_then_deliver_stream_failure(|| ffi::validate_with_policy(self.0, policy))
    }

    pub fn data_to_sign(&self) -> Result<Vec<u8>, Error> {
        call_then_deliver_stream_failure(|| ffi::data_to_sign(self.0))
    }

    pub fn policy(&self) -> Result<String, Error> {
        Ok(ffi::policy(self.0)?)
    }

    pub fn sp_uri(&self) -> Result<String, Error> {
        Ok(ffi::sp_uri(self.0)?)
    }

    pub fn profile(&self) -> Result<String, Error> {
        Ok(ffi::profile(self.0)?)
    }

    pub fn city(&self) -> Result<String, Error> {
        Ok(ffi::city(self.0)?)
    }

    pub fn state_or_province(&self) -> Result<String, Error> {
        Ok(ffi::state_or_province(self.0)?)
    }

    pub fn postal_code(&self) -> Result<String, Error> {
        Ok(ffi::postal_code(self.0)?)
    }

    pub fn country_name(&self) -> Result<String, Error> {
        Ok(ffi::country_name(self.0)?)
    }

    pub fn street_address(&self) -> Result<String, Error> {
        Ok(ffi::street_address(self.0)?)
    }

    pub fn signer_roles(&self) -> Result<Vec<String>, Error> {
        Ok(ffi::signer_roles(self.0)?)
    }

    pub fn ocsp_produced_at(&self) -> Result<String, Error> {
        Ok(ffi::ocsp_produced_at(self.0)?)
    }

    pub fn ocsp_certificate(&self) -> Result<Option<DerCert>, Error> {
        Ok(DerCert::from_ffi(ffi::ocsp_certificate(self.0)?))
    }

    pub fn time_stamp_certificate(&self) -> Result<Option<DerCert>, Error> {
        Ok(DerCert::from_ffi(ffi::time_stamp_certificate(self.0)?))
    }

    pub fn time_stamp_time(&self) -> Result<String, Error> {
        Ok(ffi::time_stamp_time(self.0)?)
    }

    pub fn archive_time_stamps(&self) -> Result<Vec<TSAInfo>, Error> {
        Ok(ffi::archive_time_stamps(self.0)?
            .into_iter()
            .map(TSAInfo::from_ffi)
            .collect())
    }

    pub fn signed_by(&self) -> Result<String, Error> {
        Ok(ffi::signed_by(self.0)?)
    }

    pub fn message_imprint(&self) -> Result<Vec<u8>, Error> {
        Ok(ffi::message_imprint(self.0)?)
    }

    pub fn validator_result(&self) -> Result<ValidatorResult, Error> {
        call_then_deliver_stream_failure(|| ffi::validator(self.0)).map(ValidatorResult)
    }
}

impl SignatureMut<'_> {
    #[must_use]
    pub fn as_signature(&self) -> Signature<'_> {
        Signature(self.0.as_ref().get_ref())
    }

    pub fn set_signature_value(&mut self, value: &[u8]) -> Result<(), Error> {
        Ok(ffi::set_signature_value(self.0.as_mut(), value)?)
    }

    pub fn extend_signature_profile<S>(&mut self, signer: &mut S) -> Result<(), Error>
    where
        S: Signer + ?Sized,
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
            .unwrap_or_else(|_| String::from("<unknown>"));
        let date = self
            .trusted_signing_time()
            .unwrap_or_else(|_| String::from("<unknown>"));
        let profile = self.profile().unwrap_or_else(|_| String::from("<unknown>"));
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
