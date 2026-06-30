#![allow(
    dead_code,
    reason = "each integration test binary uses a subset of these helpers"
)]

use bon::Builder;
use itertools::Itertools;
use libdigidocpp::{
    Conf, Container, DataFile, DerBytes, Error, ExceptionCode, Signature, TSAInfo, ValidatorResult,
    ValidatorStatus,
};
use std::error::Error as StdError;
use std::ffi::OsString;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

#[allow(unused_macros)]
macro_rules! assert_general_digidoc_err {
    ($val:expr $(,)?) => {
        match $val {
            result => {
                let Err(::libdigidocpp::Error::Library(::libdigidocpp::LibraryError::DigiDoc(error))) = result else {
                    ::core::panic!("Expected digidoc error, instead got {result:?}");
                };
                ::core::assert_eq!(error.code(), Some(::libdigidocpp::ExceptionCode::General));
                error
            }
        }
    };
    ($val:expr, $($arg:tt)+) => {
        match $val {
            result => {
                let Err(::libdigidocpp::Error::Library(::libdigidocpp::LibraryError::DigiDoc(error))) = result else {
                    ::core::panic!(
                        "{}: Expected digidoc error, instead got {result:?}",
                        ::core::format_args!($($arg)+)
                    );
                };
                ::core::assert_eq!(error.code(), Some(::libdigidocpp::ExceptionCode::General));
                error
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use assert_general_digidoc_err;

fn fixtures_directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

pub(crate) fn fixture_path(name: &str) -> String {
    let path = fixtures_directory().join(name);
    assert!(
        path.exists(),
        "Path {} doesn't exist",
        path.to_str().expect("Can't produce string from path")
    );

    path.into_os_string()
        .into_string()
        .expect("Can't produce string from path")
}

pub(crate) fn path_to_string(path: impl AsRef<Path>) -> String {
    path.as_ref()
        .to_str()
        .expect("Can't produce string from path")
        .to_owned()
}

pub(crate) fn conf_from_fixture(name: &str) -> Result<Conf, Box<Error>> {
    Ok(Conf::from_file(fixture_path(name))?)
}

// defend aganst libdigidocpp writing to user dir during test
pub(crate) fn copy_fixtures(directory: &Path, names: &[&str]) -> Result<(), Box<dyn StdError>> {
    for name in names {
        fs::copy(fixture_path(name), directory.join(name))?;
    }

    Ok(())
}

pub(crate) fn offline_conf(directory: &Path, tsl_name: &str) -> Result<Conf, Box<dyn StdError>> {
    // if we dont provide file, `libdigidocpp` searches system dirs
    let mut conf = conf_from_fixture("empty.conf")?;
    conf.set_log_file(directory.join("libdigidocpp.log"))?;
    conf.set_tsl_url(path_to_string(directory.join(tsl_name)))?;
    // `libdigidocpp` writes here
    conf.set_tsl_cache(directory)?;
    conf.set_tsl_auto_update(false)?;
    conf.set_tsl_online_digest(false)?;
    conf.set_tsl_allow_expired(true)?;
    conf.add_tsl_cert(fs::read(fixture_path("TSL.der"))?)?;

    Ok(conf)
}

pub(crate) fn initialize_offline(
    tsl_name: &str,
    other_fixtures: &[&str],
) -> Result<TempDir, Box<dyn StdError>> {
    let directory = TempDir::new()?;
    copy_fixtures(directory.path(), &[&[tsl_name], other_fixtures].concat())?;
    libdigidocpp::init_conf(offline_conf(directory.path(), tsl_name)?)?;
    libdigidocpp::initialize()?;

    Ok(directory)
}

pub(crate) fn validate_tsl_asice_with(
    tsl_name: &str,
) -> Result<Result<(), Error>, Box<dyn StdError>> {
    let _temp_dir = initialize_offline(tsl_name, &[])?;
    let container = Container::open(fixture_path("tsl.asice"))?;
    let signatures = container.signatures()?;
    let signature = signatures
        .first()
        .expect("tsl.asice must contain signature");

    Ok(signature.validate())
}

fn parse_common_name(der: &DerBytes) -> Result<String, Box<dyn StdError>> {
    let (_, certificate) = x509_parser::parse_x509_certificate(&der.0)?;
    let common_name = certificate
        .subject()
        .iter_common_name()
        .next()
        .ok_or("no common name")?
        .as_str()?;

    Ok(common_name.to_owned())
}

pub(crate) fn assert_if_some<T: PartialEq + Debug + ?Sized>(left: &T, right: Option<&T>) {
    if let Some(value) = right {
        assert_eq!(left, value);
    }
}

#[derive(Builder)]
#[builder(on(_, into))]
pub(crate) struct ContainerExpectation {
    pub(crate) media_type: Option<String>,
    pub(crate) data_files: Option<Vec<DataFileExpectation>>,
    pub(crate) signatures: Option<Vec<SignatureExpectation>>,
}

impl ContainerExpectation {
    pub(crate) const MEDIA_TYPE_ASICE: &str = "application/vnd.etsi.asic-e+zip";
    pub(crate) const MEDIA_TYPE_ASICS: &str = "application/vnd.etsi.asic-s+zip";

    pub(crate) fn assert(&self, container: &Container) -> Result<(), Box<dyn StdError>> {
        if let Some(expected_media_type) = &self.media_type {
            assert_eq!(&container.media_type()?, expected_media_type);
        }

        if let Some(expected_data_files) = &self.data_files {
            let data_files = container.data_files()?;
            assert_eq!(data_files.len(), expected_data_files.len());
            expected_data_files
                .iter()
                .zip_eq(data_files)
                .try_for_each(|(expectation, data_file)| expectation.assert(&data_file))?;
        }

        if let Some(expected_signatures) = &self.signatures {
            let signatures = container.signatures()?;
            assert_eq!(signatures.len(), expected_signatures.len());
            expected_signatures
                .iter()
                .zip_eq(signatures)
                .try_for_each(|(expectation, signature)| expectation.assert(&signature))?;
        }

        Ok(())
    }
}

#[derive(Builder)]
#[builder(on(_, into))]
pub(crate) struct DataFileExpectation {
    pub(crate) id: String,
    pub(crate) file_name: String,
    pub(crate) file_size: u64,
    pub(crate) media_type: String,
    pub(crate) calc_digest_default: Option<Vec<u8>>,
}

impl DataFileExpectation {
    pub(crate) const MEDIA_TYPE_OCTET_STREAM: &str = "application/octet-stream";
    pub(crate) const MEDIA_TYPE_TEXT_PLAIN: &str = "text/plain";

    pub(crate) fn assert(&self, data_file: &DataFile) -> Result<(), Box<dyn StdError>> {
        assert_eq!(data_file.id()?, self.id);
        assert_eq!(data_file.file_name()?, self.file_name);
        assert_eq!(data_file.file_size()?, self.file_size);
        assert_eq!(data_file.media_type()?, self.media_type);
        assert_if_some(
            &data_file.calc_digest_default()?,
            self.calc_digest_default.as_ref(),
        );
        Ok(())
    }
}
#[derive(Builder)]
#[builder(on(_, into))]
pub(crate) struct SignatureExpectation {
    pub(crate) signed_by: String,
    pub(crate) id: String,
    pub(crate) claimed_signing_time: String,
    pub(crate) trusted_signing_time: String,
    pub(crate) signature_method: String,
    pub(crate) policy: String,
    pub(crate) sp_uri: String,
    pub(crate) profile: String,
    pub(crate) city: String,
    pub(crate) state_or_province: String,
    pub(crate) postal_code: String,
    pub(crate) country_name: String,
    pub(crate) street_address: String,
    pub(crate) signer_roles: Vec<String>,
    pub(crate) ocsp_produced_at: String,
    pub(crate) time_stamp_time: String,
    pub(crate) validate_ok: bool,
    pub(crate) validator_result: ValidatorResultExpectation,
    pub(crate) archive_time_stamps: Vec<TsaInfoExpectation>,
    pub(crate) signing_certificate: Option<Vec<u8>>,
    pub(crate) signing_certificate_common_name: Option<String>,
    pub(crate) data_to_sign: Option<Vec<u8>>,
    pub(crate) ocsp_certificate: Option<Vec<u8>>,
    pub(crate) time_stamp_certificate: Option<Vec<u8>>,
    pub(crate) time_stamp_certificate_common_name: Option<String>,
    pub(crate) message_imprint: Option<Vec<u8>>,
}

impl SignatureExpectation {
    pub(crate) fn assert(&self, signature: &Signature) -> Result<(), Box<dyn StdError>> {
        assert_eq!(signature.signed_by()?, self.signed_by);
        assert_eq!(signature.id()?, self.id);
        assert_eq!(signature.claimed_signing_time()?, self.claimed_signing_time);
        assert_eq!(signature.trusted_signing_time()?, self.trusted_signing_time);
        assert_eq!(signature.signature_method()?, self.signature_method);
        assert_eq!(signature.policy()?, self.policy);
        assert_eq!(signature.sp_uri()?, self.sp_uri);
        assert_eq!(signature.profile()?, self.profile);
        assert_eq!(signature.city()?, self.city);
        assert_eq!(signature.state_or_province()?, self.state_or_province);
        assert_eq!(signature.postal_code()?, self.postal_code);
        assert_eq!(signature.country_name()?, self.country_name);
        assert_eq!(signature.street_address()?, self.street_address);
        assert_eq!(signature.signer_roles()?, self.signer_roles);
        assert_eq!(signature.ocsp_produced_at()?, self.ocsp_produced_at);
        assert_eq!(signature.time_stamp_time()?, self.time_stamp_time);
        assert_eq!(signature.validate().is_ok(), self.validate_ok);

        self.validator_result.assert(&signature.validator_result()?);

        let archive_time_stamps = &signature.archive_time_stamps()?;
        assert_eq!(archive_time_stamps.len(), self.archive_time_stamps.len());
        self.archive_time_stamps
            .iter()
            .zip_eq(archive_time_stamps)
            .for_each(|(expectation, tsa_info)| expectation.assert(tsa_info));

        assert_if_some(
            &signature.signing_certificate()?.0,
            self.signing_certificate.as_ref(),
        );
        assert_if_some(&signature.data_to_sign()?, self.data_to_sign.as_ref());
        assert_if_some(
            &signature.ocsp_certificate()?.0,
            self.ocsp_certificate.as_ref(),
        );
        assert_if_some(
            &signature.time_stamp_certificate()?.0,
            self.time_stamp_certificate.as_ref(),
        );
        assert_if_some(&signature.message_imprint()?, self.message_imprint.as_ref());
        if let Some(expected) = &self.signing_certificate_common_name {
            assert_eq!(
                &parse_common_name(&signature.signing_certificate()?)?,
                expected
            );
        }
        if let Some(expected) = &self.time_stamp_certificate_common_name {
            assert_eq!(
                &parse_common_name(&signature.time_stamp_certificate()?)?,
                expected
            );
        }

        Ok(())
    }
}

#[derive(Builder)]
#[builder(on(_, into))]
pub(crate) struct TsaInfoExpectation {
    pub(crate) cert: Option<Vec<u8>>,
    pub(crate) time: String,
}

impl TsaInfoExpectation {
    pub(crate) fn assert(&self, tsa_info: &TSAInfo) {
        assert_eq!(tsa_info.time(), self.time);
        assert_if_some(tsa_info.cert(), self.cert.as_deref());
    }
}

#[derive(Builder)]
#[builder(on(_, into))]
pub(crate) struct ValidatorResultExpectation {
    pub(crate) status: ValidatorStatus,
    pub(crate) diagnostics: Option<String>,
    pub(crate) warnings: Vec<ExceptionCode>,
}

impl ValidatorResultExpectation {
    pub(crate) fn assert(&self, validator_result: &ValidatorResult) {
        assert_eq!(validator_result.status(), self.status);
        assert_if_some(validator_result.diagnostics(), self.diagnostics.as_deref());
        let expected_warnings = self
            .warnings
            .iter()
            .copied()
            .map(|code| (Some(code), i32::from(code)))
            .collect::<Vec<_>>();
        assert_eq!(validator_result.warnings(), expected_warnings);
    }
}

#[cfg(unix)]
pub(crate) fn non_utf8_path() -> PathBuf {
    use std::os::unix::ffi::OsStringExt;

    PathBuf::from(OsString::from_vec(b"invalid-\xff.asice".to_vec()))
}

#[cfg(windows)]
pub(crate) fn non_utf8_path() -> PathBuf {
    use std::os::windows::ffi::OsStringExt;

    // 0xD800 is a lone surrogate, so it is not valid UTF-16.
    PathBuf::from(OsString::from_wide(&[0x0069, 0xD800, 0x002E, 0x0061]))
}
