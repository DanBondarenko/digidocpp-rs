use derive_more::From;
use std::convert::Infallible;
use std::str::FromStr;
use strum::{AsRefStr, Display, EnumString, ParseError};

/// A cryptographic hash function, named by its XML-DSig URI.
///
/// String form (`ToString`/`FromStr`) is the XML-DSig algorithm URI. SHA-1 is weak: signatures that use it
/// validate with a warning.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, AsRefStr, Display, EnumString)]
#[non_exhaustive]
pub enum DigestAlgorithm {
    /// SHA-1.
    #[strum(serialize = "http://www.w3.org/2000/09/xmldsig#sha1")]
    Sha1,
    /// SHA-224.
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#sha224")]
    Sha224,
    /// SHA-256.
    #[strum(serialize = "http://www.w3.org/2001/04/xmlenc#sha256")]
    Sha256,
    /// SHA-384.
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#sha384")]
    Sha384,
    /// SHA-512.
    #[strum(serialize = "http://www.w3.org/2001/04/xmlenc#sha512")]
    Sha512,
    /// SHA3-224.
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-224")]
    Sha3_224,
    /// SHA3-256.
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-256")]
    Sha3_256,
    /// SHA3-384.
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-384")]
    Sha3_384,
    /// SHA3-512.
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-512")]
    Sha3_512,
}

/// A digital signature algorithm, named by its XML-DSig URI.
///
/// Text form (`ToString`/`FromStr`) is the XML-DSig algorithm URI. The `Sha1` variants are weak: signatures
/// that use them validate with a warning.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, AsRefStr, Display, EnumString)]
#[non_exhaustive]
pub enum SignatureAlgorithm {
    /// RSA PKCS #1 v1.5 with SHA-1.
    #[strum(serialize = "http://www.w3.org/2000/09/xmldsig#rsa-sha1")]
    RsaSha1,
    /// RSA PKCS #1 v1.5 with SHA-224.
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha224")]
    RsaSha224,
    /// RSA PKCS #1 v1.5 with SHA-256.
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256")]
    RsaSha256,
    /// RSA PKCS #1 v1.5 with SHA-384.
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha384")]
    RsaSha384,
    /// RSA PKCS #1 v1.5 with SHA-512.
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha512")]
    RsaSha512,
    /// RSA-PSS (MGF1) with SHA-224.
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha224-rsa-MGF1")]
    RsaPssSha224,
    /// RSA-PSS (MGF1) with SHA-256.
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha256-rsa-MGF1")]
    RsaPssSha256,
    /// RSA-PSS (MGF1) with SHA-384.
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha384-rsa-MGF1")]
    RsaPssSha384,
    /// RSA-PSS (MGF1) with SHA-512.
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha512-rsa-MGF1")]
    RsaPssSha512,
    /// RSA-PSS (MGF1) with SHA3-224.
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-224-rsa-MGF1")]
    RsaPssSha3_224,
    /// RSA-PSS (MGF1) with SHA3-256.
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-256-rsa-MGF1")]
    RsaPssSha3_256,
    /// RSA-PSS (MGF1) with SHA3-384.
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-384-rsa-MGF1")]
    RsaPssSha3_384,
    /// RSA-PSS (MGF1) with SHA3-512.
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-512-rsa-MGF1")]
    RsaPssSha3_512,
    /// ECDSA with SHA-1.
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha1")]
    EcdsaSha1,
    /// ECDSA with SHA-224.
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha224")]
    EcdsaSha224,
    /// ECDSA with SHA-256.
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha256")]
    EcdsaSha256,
    /// ECDSA with SHA-384.
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha384")]
    EcdsaSha384,
    /// ECDSA with SHA-512.
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha512")]
    EcdsaSha512,
}

/// Hash function choice for [`DataFile::calc_digest`](crate::DataFile::calc_digest).
///
/// String form (`ToString`/`FromStr`) is the XML-DSig algorithm URI or an empty string for
/// [`Default`](Self::Default).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, AsRefStr, Display, From)]
#[non_exhaustive]
pub enum DigestSetting {
    /// The library default, configured via [`Conf::digest_uri`](crate::Conf::digest_uri).
    #[default]
    #[strum(serialize = "")]
    #[from(skip)]
    Default,
    /// A specific hash function.
    #[strum(transparent)]
    Digest(DigestAlgorithm),
}

impl FromStr for DigestSetting {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.is_empty() {
            Ok(Self::Default)
        } else {
            text.parse().map(Self::Digest)
        }
    }
}

/// Algorithm choice for [`PartialSigner::set_method`](crate::PartialSigner::set_method).
///
/// String form (`ToString`/`FromStr`) is the XML-DSig algorithm URI or an empty string for
/// [`Default`](Self::Default).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, AsRefStr, Display, From)]
#[non_exhaustive]
pub enum AlgorithmSetting {
    /// The signer default ([`PartialSigner::method`](crate::PartialSigner::method)).
    #[default]
    #[strum(serialize = "")]
    #[from(skip)]
    Default,
    /// A hash function. libdigidocpp pairs it with the signer's key type: RSA PKCS #1 v1.5 or ECDSA.
    ///
    /// A SHA-3 hash function gives RSA-PSS and needs a RSA key.
    #[strum(transparent)]
    Digest(DigestAlgorithm),
    /// An exact signature algorithm. It must match the key type.
    #[strum(transparent)]
    Signature(SignatureAlgorithm),
}

impl FromStr for AlgorithmSetting {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.is_empty() {
            Ok(Self::Default)
        } else {
            DigestAlgorithm::from_str(text)
                .map(Self::Digest)
                .or_else(|_| SignatureAlgorithm::from_str(text).map(Self::Signature))
        }
    }
}

/// A hash function or signature algorithm.
///
/// Returned where libdigidocpp reports an algorithm, for example by
/// [`Signature::signature_method`](crate::Signature::signature_method).
///
/// String form (`ToString`/`FromStr`) is the XML-DSig algorithm URI. An unknown URI returned by
/// upstream becomes [`Other`](Self::Other).
#[derive(Clone, Debug, Eq, Hash, PartialEq, AsRefStr, Display, From)]
#[non_exhaustive]
pub enum Algorithm {
    /// A hash function.
    #[strum(transparent)]
    Digest(DigestAlgorithm),
    /// A signature algorithm.
    #[strum(transparent)]
    Signature(SignatureAlgorithm),
    /// An algorithm URI this crate does not know.
    #[strum(transparent)]
    #[from(skip)]
    Other(String),
}

/// Unknown text becomes [`Algorithm::Other`].
impl From<&str> for Algorithm {
    fn from(text: &str) -> Self {
        DigestAlgorithm::from_str(text)
            .map(Self::Digest)
            .or_else(|_| SignatureAlgorithm::from_str(text).map(Self::Signature))
            .unwrap_or_else(|_| Self::Other(text.to_owned()))
    }
}

impl FromStr for Algorithm {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(text))
    }
}

/// Profile choice for [`PartialSigner::set_profile`](crate::PartialSigner::set_profile).
///
/// String form (`ToString`/`FromStr`) is the profile name or an empty string for
/// [`Default`](Self::Default).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, AsRefStr, Display, EnumString)]
#[non_exhaustive]
pub enum ProfileSetting {
    /// The libdigidocpp default.
    #[default]
    #[strum(serialize = "")]
    Default,
    /// Basic signature, no time-stamp or revocation data. ASiC-E only.
    #[strum(serialize = "BES")]
    Bes,
    /// Signature that cites a published set of signing rules. No time-stamp or revocation data. ASiC-E only.
    #[strum(serialize = "EPES")]
    Epes,
    /// Signature time-stamp and OCSP response. ASiC-E only.
    #[strum(to_string = "time-stamp", serialize = "TS")]
    TimeStamp,
    /// [`TimeStamp`](Self::TimeStamp) plus an archive time-stamp. ASiC-E only.
    #[strum(to_string = "time-stamp-archive", serialize = "TSA")]
    TimeStampArchive,
    /// Time-stamp token of an ASiC-S container. ASiC-S only.
    #[strum(to_string = "TimeStampToken", serialize = "time-stamp-token")]
    TimeStampToken,
}

/// Profile reported by [`PartialSigner::profile`](crate::PartialSigner::profile).
///
/// String form (`ToString`/`FromStr`) is the profile name.
/// An unknown profile returned by upstream becomes [`Other`](Self::Other).
#[derive(Clone, Debug, Eq, Hash, PartialEq, AsRefStr, Display, EnumString)]
#[non_exhaustive]
pub enum SigningProfile {
    /// Basic signature, no time-stamp or revocation data. ASiC-E only.
    #[strum(serialize = "BES")]
    Bes,
    /// Signature that cites a published set of signing rules. No time-stamp or revocation data. ASiC-E only.
    #[strum(serialize = "EPES")]
    Epes,
    /// Signature time-stamp and OCSP response. ASiC-E only.
    #[strum(serialize = "time-stamp")]
    TimeStamp,
    /// [`TimeStamp`](Self::TimeStamp) plus an archive time-stamp. ASiC-E only.
    #[strum(serialize = "time-stamp-archive")]
    TimeStampArchive,
    /// Time-stamp token of an ASiC-S container. ASiC-S only.
    #[strum(serialize = "TimeStampToken")]
    TimeStampToken,
    /// A profile name this crate does not know.
    #[strum(default, transparent)]
    Other(String),
}

/// Profile reported by [`Signature::profile`](crate::Signature::profile).
///
/// String form (`ToString`/`FromStr`) is the profile name. An unknown profile returned by
/// upstream becomes [`Other`](Self::Other).
#[derive(Clone, Debug, Eq, Hash, PartialEq, AsRefStr, Display, EnumString)]
#[non_exhaustive]
pub enum SignatureProfile {
    /// Basic signature, no time-stamp or revocation data.
    #[strum(serialize = "BES")]
    Bes,
    /// [`Bes`](Self::Bes) plus a signature time-stamp.
    #[strum(serialize = "BES/time-stamp")]
    BesTimeStamp,
    /// [`BesTimeStamp`](Self::BesTimeStamp) plus an archive time-stamp.
    #[strum(serialize = "BES/time-stamp-archive")]
    BesTimeStampArchive,
    /// [`Bes`](Self::Bes) plus an OCSP response used as time-mark.
    #[strum(serialize = "BES/time-mark")]
    BesTimeMark,
    /// [`BesTimeMark`](Self::BesTimeMark) plus an archive time-stamp.
    #[strum(serialize = "BES/time-mark-archive")]
    BesTimeMarkArchive,
    /// Signature with an explicit policy, no time-stamp or revocation data.
    #[strum(serialize = "EPES")]
    Epes,
    /// [`Epes`](Self::Epes) plus a signature time-stamp.
    #[strum(serialize = "EPES/time-stamp")]
    EpesTimeStamp,
    /// [`EpesTimeStamp`](Self::EpesTimeStamp) plus an archive time-stamp.
    #[strum(serialize = "EPES/time-stamp-archive")]
    EpesTimeStampArchive,
    /// [`Epes`](Self::Epes) plus an OCSP response used as time-mark.
    #[strum(serialize = "EPES/time-mark")]
    EpesTimeMark,
    /// [`EpesTimeMark`](Self::EpesTimeMark) plus an archive time-stamp.
    #[strum(serialize = "EPES/time-mark-archive")]
    EpesTimeMarkArchive,
    /// Time-stamp token of an ASiC-S container.
    #[strum(serialize = "TimeStampToken")]
    TimeStampToken,
    /// PAdES baseline B: basic signature. Reported for PDF files validated through SiVa.
    #[strum(serialize = "PAdES_BASELINE_B")]
    PadesBaselineB,
    /// PAdES baseline B, as named by libdigidocpp's own PDF support. SiVa uses
    /// [`PadesBaselineB`](Self::PadesBaselineB).
    #[strum(serialize = "PAdES_BASELINE_BES")]
    PadesBaselineBes,
    /// PAdES baseline T: with a signature time-stamp. Reported for PDF files validated through
    /// SiVa.
    #[strum(serialize = "PAdES_BASELINE_T")]
    PadesBaselineT,
    /// PAdES baseline LT: with long-term validation data. Reported for PDF files validated
    /// through SiVa.
    #[strum(serialize = "PAdES_BASELINE_LT")]
    PadesBaselineLt,
    /// PAdES baseline LTA: [`PadesBaselineLt`](Self::PadesBaselineLt) plus an archive
    /// time-stamp. Reported for PDF files validated through SiVa.
    #[strum(serialize = "PAdES_BASELINE_LTA")]
    PadesBaselineLta,
    /// A profile name this crate does not know.
    #[strum(default, transparent)]
    Other(String),
}
