use derive_more::From;
use std::convert::Infallible;
use std::str::FromStr;
use strum::{AsRefStr, Display, EnumString, ParseError};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, AsRefStr, Display, EnumString)]
#[non_exhaustive]
pub enum DigestAlgorithm {
    #[strum(serialize = "http://www.w3.org/2000/09/xmldsig#sha1")]
    Sha1,
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#sha224")]
    Sha224,
    #[strum(serialize = "http://www.w3.org/2001/04/xmlenc#sha256")]
    Sha256,
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#sha384")]
    Sha384,
    #[strum(serialize = "http://www.w3.org/2001/04/xmlenc#sha512")]
    Sha512,
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-224")]
    Sha3_224,
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-256")]
    Sha3_256,
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-384")]
    Sha3_384,
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-512")]
    Sha3_512,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, AsRefStr, Display, EnumString)]
#[non_exhaustive]
pub enum SignatureAlgorithm {
    #[strum(serialize = "http://www.w3.org/2000/09/xmldsig#rsa-sha1")]
    RsaSha1,
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha224")]
    RsaSha224,
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256")]
    RsaSha256,
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha384")]
    RsaSha384,
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha512")]
    RsaSha512,
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha224-rsa-MGF1")]
    RsaPssSha224,
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha256-rsa-MGF1")]
    RsaPssSha256,
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha384-rsa-MGF1")]
    RsaPssSha384,
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha512-rsa-MGF1")]
    RsaPssSha512,
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-224-rsa-MGF1")]
    RsaPssSha3_224,
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-256-rsa-MGF1")]
    RsaPssSha3_256,
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-384-rsa-MGF1")]
    RsaPssSha3_384,
    #[strum(serialize = "http://www.w3.org/2007/05/xmldsig-more#sha3-512-rsa-MGF1")]
    RsaPssSha3_512,
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha1")]
    EcdsaSha1,
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha224")]
    EcdsaSha224,
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha256")]
    EcdsaSha256,
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha384")]
    EcdsaSha384,
    #[strum(serialize = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha512")]
    EcdsaSha512,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, AsRefStr, Display, From)]
#[non_exhaustive]
pub enum DigestSetting {
    #[default]
    #[strum(serialize = "")]
    #[from(skip)]
    Default,
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, AsRefStr, Display, From)]
#[non_exhaustive]
pub enum AlgorithmSetting {
    #[default]
    #[strum(serialize = "")]
    #[from(skip)]
    Default,
    #[strum(transparent)]
    Digest(DigestAlgorithm),
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

#[derive(Clone, Debug, Eq, Hash, PartialEq, AsRefStr, Display, From)]
#[non_exhaustive]
pub enum Algorithm {
    #[strum(transparent)]
    Digest(DigestAlgorithm),
    #[strum(transparent)]
    Signature(SignatureAlgorithm),
    #[strum(transparent)]
    #[from(skip)]
    Other(String),
}

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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, AsRefStr, Display, EnumString)]
#[non_exhaustive]
pub enum ProfileSetting {
    #[default]
    #[strum(serialize = "")]
    Default,
    #[strum(serialize = "BES")]
    Bes,
    #[strum(serialize = "EPES")]
    Epes,
    #[strum(to_string = "time-stamp", serialize = "TS")]
    TimeStamp,
    #[strum(to_string = "time-stamp-archive", serialize = "TSA")]
    TimeStampArchive,
    #[strum(to_string = "TimeStampToken", serialize = "time-stamp-token")]
    TimeStampToken,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, AsRefStr, Display, EnumString)]
#[non_exhaustive]
pub enum SigningProfile {
    #[strum(serialize = "BES")]
    Bes,
    #[strum(serialize = "EPES")]
    Epes,
    #[strum(serialize = "time-stamp")]
    TimeStamp,
    #[strum(serialize = "time-stamp-archive")]
    TimeStampArchive,
    #[strum(serialize = "TimeStampToken")]
    TimeStampToken,
    #[strum(default, transparent)]
    Other(String),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, AsRefStr, Display, EnumString)]
#[non_exhaustive]
pub enum SignatureProfile {
    #[strum(serialize = "BES")]
    Bes,
    #[strum(serialize = "BES/time-stamp")]
    BesTimeStamp,
    #[strum(serialize = "BES/time-stamp-archive")]
    BesTimeStampArchive,
    #[strum(serialize = "BES/time-mark")]
    BesTimeMark,
    #[strum(serialize = "BES/time-mark-archive")]
    BesTimeMarkArchive,
    #[strum(serialize = "EPES")]
    Epes,
    #[strum(serialize = "EPES/time-stamp")]
    EpesTimeStamp,
    #[strum(serialize = "EPES/time-stamp-archive")]
    EpesTimeStampArchive,
    #[strum(serialize = "EPES/time-mark")]
    EpesTimeMark,
    #[strum(serialize = "EPES/time-mark-archive")]
    EpesTimeMarkArchive,
    #[strum(serialize = "TimeStampToken")]
    TimeStampToken,
    #[strum(serialize = "PAdES_BASELINE_B")]
    PadesBaselineB,
    #[strum(serialize = "PAdES_BASELINE_BES")]
    PadesBaselineBes,
    #[strum(serialize = "PAdES_BASELINE_T")]
    PadesBaselineT,
    #[strum(serialize = "PAdES_BASELINE_LT")]
    PadesBaselineLt,
    #[strum(serialize = "PAdES_BASELINE_LTA")]
    PadesBaselineLta,
    #[strum(default, transparent)]
    Other(String),
}
