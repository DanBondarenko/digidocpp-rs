#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::redundant_pub_crate
    )
)]
mod common;

use crate::common::{ContainerExpectation, SignatureExpectation, ValidatorResultExpectation};
use libdigidocpp::{Container, ExceptionCode, ValidatorStatus};
use serial_test::serial;
use std::error::Error as StdError;

#[test]
#[serial]
fn reads_expired_signature_from_asice_container() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .signatures([expired_signature()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("test.asice"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_signature_from_asics_container() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .signatures([valid_signature_a()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("test.asics"));
    container_expectation.assert(&container?)
}

#[test]
#[serial]
fn reads_signature_from_asics_container_with_metadata() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .signatures([valid_signature_a()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("test-meta.asics"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_signature_with_invalid_timestamp_token_from_asics_container()
-> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .signatures([signature_with_invalid_timestamp()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("test-invalidts.asics"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_signatures_from_asics_container_with_zero_size_contents() -> Result<(), Box<dyn StdError>>
{
    let container_expectation = ContainerExpectation::builder()
        .signatures([valid_signature_b()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("test-tera-empty.asics"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_signatures_from_asics_container_with_large_contents() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .signatures([valid_signature_c()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("test-tera.asics"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_signatures_from_asice_container_with_subfolder_contents() -> Result<(), Box<dyn StdError>>
{
    let container_expectation = ContainerExpectation::builder().signatures([]).build();

    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let container = Container::open(common::fixture_path("asice-path.asice"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_forged_signature_from_asice_container() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .signatures([forged_signature()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("forged_lt.asice"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_signature_from_asice_container() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .signatures([valid_signature_d()])
        .build();

    let _temp_dir = common::initialize_offline("EE_T.xml", &[])?;
    let container = Container::open(common::fixture_path("tsl.asice"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn created_empty_asice_container_has_no_signatures() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let container = Container::create("new.asice")?;

    ContainerExpectation::builder()
        .signatures([])
        .build()
        .assert(&container)
}

const PROFILE_BES_TIME_STAMP: &str = "BES/time-stamp";
const PROFILE_TIME_STAMP_TOKEN: &str = "TimeStampToken";
const SIGNED_BY_DEMO: &str = "DEMO of SK TSA 2014";

pub(crate) fn valid_signature_a() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by(SIGNED_BY_DEMO)
        .id("8766262679921277358")
        .time_stamp_certificate_common_name(SIGNED_BY_DEMO)
        .claimed_signing_time("2016-11-02T11:07:45Z")
        .trusted_signing_time("2016-11-02T11:07:45Z")
        .signature_method(common::URI_SHA256)
        .policy("")
        .sp_uri("")
        .profile(PROFILE_TIME_STAMP_TOKEN)
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
pub(crate) fn valid_signature_b() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by(SIGNED_BY_DEMO)
        .id("5641640388766379252")
        .claimed_signing_time("2017-04-10T06:27:29Z")
        .trusted_signing_time("2017-04-10T06:27:29Z")
        .signature_method(common::URI_SHA256)
        .policy("")
        .sp_uri("")
        .profile(PROFILE_TIME_STAMP_TOKEN)
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
pub(crate) fn valid_signature_c() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by(SIGNED_BY_DEMO)
        .id("3198319511793657073")
        .time_stamp_certificate_common_name(SIGNED_BY_DEMO)
        .claimed_signing_time("2017-04-10T06:27:28Z")
        .trusted_signing_time("2017-04-10T06:27:28Z")
        .signature_method(common::URI_SHA256)
        .policy("")
        .sp_uri("")
        .profile(PROFILE_TIME_STAMP_TOKEN)
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
pub(crate) fn valid_signature_d() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by("MÄNNIK,MARI-LIIS,47101010033")
        .id("S0")
        .claimed_signing_time("2016-11-28T13:46:41Z")
        .trusted_signing_time("2016-11-28T13:46:43Z")
        .signature_method("http://www.w3.org/2001/04/xmldsig-more#rsa-sha256")
        .policy("")
        .sp_uri("")
        .profile(PROFILE_BES_TIME_STAMP)
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
pub(crate) fn signature_with_invalid_timestamp() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by(SIGNED_BY_DEMO)
        .id("8766262679921277358")
        .time_stamp_certificate_common_name(SIGNED_BY_DEMO)
        .claimed_signing_time("2016-11-02T11:07:45Z")
        .trusted_signing_time("2016-11-02T11:07:45Z")
        .signature_method(common::URI_SHA256)
        .policy("")
        .sp_uri("")
        .profile(PROFILE_TIME_STAMP_TOKEN)
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
pub(crate) fn forged_signature() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by("MÖLDER,HUGO MARTIN,38910239121")
        .signing_certificate_common_name("MÖLDER,HUGO MARTIN,38910239121")
        .id("S0")
        .claimed_signing_time("2026-05-28T07:26:06Z")
        .trusted_signing_time("2026-05-28T07:26:07Z")
        .signature_method("http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha256")
        .policy("")
        .sp_uri("")
        .profile(PROFILE_BES_TIME_STAMP)
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
pub(crate) fn expired_signature() -> SignatureExpectation {
    SignatureExpectation::builder()
        .signed_by("signer1")
        .id("S0")
        .claimed_signing_time("2013-04-06T13:42:47Z")
        .trusted_signing_time("2013-04-06T13:42:48Z")
        .signature_method("http://www.w3.org/2001/04/xmldsig-more#rsa-sha256")
        .policy("urn:oid:1.3.6.1.4.1.10015.1000.3.1.1")
        .sp_uri("https://www.sk.ee/repository/bdoc-spec.pdf")
        .profile("EPES/time-mark")
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
