#![allow(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        unreachable_pub,
        clippy::used_underscore_binding,
    )
)]
mod common;

use crate::common::{create_empty_asice_container, init_with_valid_tsl, parse_common_name};
use bon::Builder;
use digidocpp::algorithm::{Algorithm, DigestAlgorithm, SignatureAlgorithm, SignatureProfile};
use digidocpp::error::ExceptionCode;
use digidocpp::signature::{TSAInfo, ValidatorResult, ValidatorStatus};
use digidocpp::{Container, DerCert, Signature};
use itertools::Itertools;
use rstest::rstest;
use serial_test::serial;
use std::error::Error as StdError;
use std::fmt::Debug;
use tempfile::TempDir;

#[rstest]
#[case::asice_with_expired_certificates("test.asice", vec![expired_signature()])]
#[case::asics("test.asics", vec![valid_signature_a()])]
#[case::asics_with_metadata("test-meta.asics", vec![valid_signature_a()])]
#[case::asics_with_invalid_timestamp_token(
    "test-invalidts.asics",
    vec![signature_with_invalid_timestamp()]
)]
#[case::asics_with_zero_size_contents("test-tera-empty.asics", vec![valid_signature_b()])]
#[case::asics_with_large_contents("test-tera.asics", vec![valid_signature_c()])]
#[case::asice_with_subfolder_contents("asice-path.asice", vec![])]
#[case::asice_with_forged_signature("forged_lt.asice", vec![forged_signature()])]
#[case::asice_for_tsl_validation("tsl.asice", vec![valid_signature_d()])]
#[serial]
fn reads_signatures_from_containers(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[case] fixture: &str,
    #[case] signatures: Vec<SignatureExpectation>,
) -> Result<(), Box<dyn StdError>> {
    let container = Container::open(common::fixture_path(fixture))?;

    assert_signatures(&container, &signatures)
}

#[rstest]
#[serial]
fn empty_asice_container_has_no_signatures(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[from(create_empty_asice_container)] container: Container,
) -> Result<(), Box<dyn StdError>> {
    assert_signatures(&container, &[])
}

const SIGNED_BY_DEMO: &str = "DEMO of SK TSA 2014";

fn valid_signature_a() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by(SIGNED_BY_DEMO)
        .id("8766262679921277358")
        .time_stamp_certificate_common_name(SIGNED_BY_DEMO)
        .claimed_signing_time("2016-11-02T11:07:45Z")
        .trusted_signing_time("2016-11-02T11:07:45Z")
        .signature_method(DigestAlgorithm::Sha256)
        .policy("")
        .sp_uri("")
        .profile(SignatureProfile::TimeStampToken)
        .city("")
        .state_or_province("")
        .postal_code("")
        .country_name("")
        .street_address("")
        .signer_roles([])
        .ocsp_produced_at("")
        .time_stamp_time("2016-11-02T11:07:45Z")
        .validate_ok(true)
        .validator_result(
            ValidatorResultExpectation::builder()
                .status(ValidatorStatus::Valid)
                .warnings([])
                .build(),
        )
        .archive_time_stamps([])
        .build()
}
fn valid_signature_b() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by(SIGNED_BY_DEMO)
        .id("5641640388766379252")
        .claimed_signing_time("2017-04-10T06:27:29Z")
        .trusted_signing_time("2017-04-10T06:27:29Z")
        .signature_method(DigestAlgorithm::Sha256)
        .policy("")
        .sp_uri("")
        .profile(SignatureProfile::TimeStampToken)
        .city("")
        .state_or_province("")
        .postal_code("")
        .country_name("")
        .street_address("")
        .signer_roles([])
        .ocsp_produced_at("")
        .time_stamp_time("2017-04-10T06:27:29Z")
        .validate_ok(true)
        .validator_result(
            ValidatorResultExpectation::builder()
                .status(ValidatorStatus::Valid)
                .warnings([])
                .build(),
        )
        .archive_time_stamps([])
        .build()
}
fn valid_signature_c() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by(SIGNED_BY_DEMO)
        .id("3198319511793657073")
        .time_stamp_certificate_common_name(SIGNED_BY_DEMO)
        .claimed_signing_time("2017-04-10T06:27:28Z")
        .trusted_signing_time("2017-04-10T06:27:28Z")
        .signature_method(DigestAlgorithm::Sha256)
        .policy("")
        .sp_uri("")
        .profile(SignatureProfile::TimeStampToken)
        .city("")
        .state_or_province("")
        .postal_code("")
        .country_name("")
        .street_address("")
        .signer_roles([])
        .ocsp_produced_at("")
        .time_stamp_time("2017-04-10T06:27:28Z")
        .validate_ok(true)
        .validator_result(
            ValidatorResultExpectation::builder()
                .status(ValidatorStatus::Valid)
                .warnings([])
                .build(),
        )
        .archive_time_stamps([])
        .build()
}
fn valid_signature_d() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by("MÄNNIK,MARI-LIIS,47101010033")
        .id("S0")
        .claimed_signing_time("2016-11-28T13:46:41Z")
        .trusted_signing_time("2016-11-28T13:46:43Z")
        .signature_method(SignatureAlgorithm::RsaSha256)
        .policy("")
        .sp_uri("")
        .profile(SignatureProfile::BesTimeStamp)
        .city("")
        .state_or_province("")
        .postal_code("")
        .country_name("")
        .street_address("")
        .signer_roles([String::from("1234")])
        .ocsp_produced_at("2016-11-28T13:46:43Z")
        .time_stamp_time("2016-11-28T13:46:43Z")
        .validate_ok(true)
        .validator_result(
            ValidatorResultExpectation::builder()
                .status(ValidatorStatus::Valid)
                .warnings([])
                .build(),
        )
        .archive_time_stamps([])
        .build()
}
fn signature_with_invalid_timestamp() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by(SIGNED_BY_DEMO)
        .id("8766262679921277358")
        .time_stamp_certificate_common_name(SIGNED_BY_DEMO)
        .claimed_signing_time("2016-11-02T11:07:45Z")
        .trusted_signing_time("2016-11-02T11:07:45Z")
        .signature_method(DigestAlgorithm::Sha256)
        .policy("")
        .sp_uri("")
        .profile(SignatureProfile::TimeStampToken)
        .city("")
        .state_or_province("")
        .postal_code("")
        .country_name("")
        .street_address("")
        .signer_roles([])
        .ocsp_produced_at("")
        .time_stamp_time("2016-11-02T11:07:45Z")
        .validate_ok(false)
        .validator_result(
            ValidatorResultExpectation::builder()
                .status(ValidatorStatus::Invalid)
                .warnings([])
                .build(),
        )
        .archive_time_stamps([])
        .build()
}
fn forged_signature() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by("MÖLDER,HUGO MARTIN,38910239121")
        .signing_certificate_common_name("MÖLDER,HUGO MARTIN,38910239121")
        .id("S0")
        .claimed_signing_time("2026-05-28T07:26:06Z")
        .trusted_signing_time("2026-05-28T07:26:07Z")
        .signature_method(SignatureAlgorithm::EcdsaSha256)
        .policy("")
        .sp_uri("")
        .profile(SignatureProfile::BesTimeStamp)
        .city("")
        .state_or_province("")
        .postal_code("")
        .country_name("")
        .street_address("")
        .signer_roles([])
        .ocsp_produced_at("2026-05-28T07:26:07Z")
        .time_stamp_time("2026-05-28T07:26:07Z")
        .validate_ok(false)
        .validator_result(
            ValidatorResultExpectation::builder()
                .status(ValidatorStatus::Invalid)
                .warnings([])
                .build(),
        )
        .archive_time_stamps([])
        .build()
}
fn expired_signature() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by("signer1")
        .id("S0")
        .claimed_signing_time("2013-04-06T13:42:47Z")
        .trusted_signing_time("2013-04-06T13:42:48Z")
        .signature_method(SignatureAlgorithm::RsaSha256)
        .policy("urn:oid:1.3.6.1.4.1.10015.1000.3.1.1")
        .sp_uri("https://www.sk.ee/repository/bdoc-spec.pdf")
        .profile(SignatureProfile::EpesTimeMark)
        .city("")
        .state_or_province("")
        .postal_code("")
        .country_name("")
        .street_address("")
        .signer_roles([])
        .ocsp_produced_at("2013-04-06T13:42:48Z")
        .time_stamp_time("")
        .validate_ok(false)
        .validator_result(
            ValidatorResultExpectation::builder()
                .status(ValidatorStatus::Unknown)
                .warnings([
                    ExceptionCode::MimeTypeWarning,
                    ExceptionCode::MimeTypeWarning,
                ])
                .build(),
        )
        .archive_time_stamps([])
        .build()
}

#[track_caller]
fn assert_if_some<T: PartialEq + Debug + ?Sized>(left: &T, right: Option<&T>) {
    if let Some(value) = right {
        assert_eq!(left, value);
    }
}

fn assert_signatures(
    container: &Container,
    expected: &[SignatureExpectation],
) -> Result<(), Box<dyn StdError>> {
    let signatures = container.signatures()?;
    assert_eq!(signatures.len(), expected.len());
    expected
        .iter()
        .zip_eq(signatures)
        .try_for_each(|(expectation, signature)| expectation.assert(signature))
}

#[derive(Builder, Debug, Clone)]
#[builder(on(_, into))]
struct SignatureExpectation {
    signed_by: String,
    id: String,
    claimed_signing_time: String,
    trusted_signing_time: String,
    signature_method: Algorithm,
    policy: String,
    sp_uri: String,
    profile: SignatureProfile,
    city: String,
    state_or_province: String,
    postal_code: String,
    country_name: String,
    street_address: String,
    signer_roles: Vec<String>,
    ocsp_produced_at: String,
    time_stamp_time: String,
    validate_ok: bool,
    validator_result: ValidatorResultExpectation,
    archive_time_stamps: Vec<TsaInfoExpectation>,
    signing_certificate: Option<DerCert>,
    signing_certificate_common_name: Option<String>,
    data_to_sign: Option<Vec<u8>>,
    ocsp_certificate: Option<DerCert>,
    time_stamp_certificate: Option<DerCert>,
    time_stamp_certificate_common_name: Option<String>,
    message_imprint: Option<Vec<u8>>,
}

impl SignatureExpectation {
    fn assert(&self, signature: Signature<'_>) -> Result<(), Box<dyn StdError>> {
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

        if let Some(expected) = &self.signing_certificate {
            assert_eq!(signature.signing_certificate()?.as_ref(), Some(expected));
        }
        assert_if_some(&signature.data_to_sign()?, self.data_to_sign.as_ref());
        if let Some(expected) = &self.ocsp_certificate {
            assert_eq!(signature.ocsp_certificate()?.as_ref(), Some(expected));
        }
        if let Some(expected) = &self.time_stamp_certificate {
            assert_eq!(signature.time_stamp_certificate()?.as_ref(), Some(expected));
        }
        assert_if_some(&signature.message_imprint()?, self.message_imprint.as_ref());
        if let Some(expected) = &self.signing_certificate_common_name {
            assert_eq!(
                &parse_common_name(&signature.signing_certificate()?.unwrap())?,
                expected
            );
        }
        if let Some(expected) = &self.time_stamp_certificate_common_name {
            assert_eq!(
                &parse_common_name(&signature.time_stamp_certificate()?.unwrap())?,
                expected
            );
        }

        Ok(())
    }
}

#[derive(Builder, Debug, Clone)]
#[builder(on(_, into))]
struct TsaInfoExpectation {
    cert: Option<DerCert>,
    time: String,
}

impl TsaInfoExpectation {
    fn assert(&self, tsa_info: &TSAInfo) {
        assert_eq!(tsa_info.time(), self.time);
        if let Some(expected) = &self.cert {
            assert_eq!(tsa_info.cert(), Some(expected));
        }
    }
}

#[derive(Builder, Debug, Clone)]
#[builder(on(_, into))]
struct ValidatorResultExpectation {
    status: ValidatorStatus,
    diagnostics: Option<String>,
    warnings: Vec<ExceptionCode>,
}

impl ValidatorResultExpectation {
    fn assert(&self, validator_result: &ValidatorResult) {
        assert_eq!(validator_result.status(), Some(self.status));
        assert_if_some(validator_result.diagnostics(), self.diagnostics.as_deref());
        #[expect(clippy::as_conversions)]
        let expected_warnings = self
            .warnings
            .iter()
            .copied()
            .map(|code| (Some(code), code as i32))
            .collect::<Vec<_>>();
        assert_eq!(validator_result.warnings(), expected_warnings);
    }
}
